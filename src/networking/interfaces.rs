use std::net::Ipv4Addr;

#[cfg(windows)]
pub fn local_ipv4_addresses() -> Vec<Ipv4Addr> {
    use std::mem::{offset_of, size_of};
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetIpAddrTable, MIB_IPADDRTABLE, MIB_IPADDRROW_XP, MIB_IPADDR_DELETED,
        MIB_IPADDR_DISCONNECTED,
    };

    const INSUFFICIENT_BUFFER: u32 = 122;
    let mut size = 0;
    // Windows reports the required byte count when no buffer is supplied.
    if unsafe { GetIpAddrTable(std::ptr::null_mut(), &mut size, 0) } != INSUFFICIENT_BUFFER {
        return Vec::new();
    }

    for _ in 0..3 {
        // u32 storage provides the alignment required by the table and its rows.
        let mut buffer = vec![0u32; (size as usize).div_ceil(size_of::<u32>())];
        let table = buffer.as_mut_ptr().cast::<MIB_IPADDRTABLE>();
        match unsafe { GetIpAddrTable(table, &mut size, 0) } {
            INSUFFICIENT_BUFFER => continue,
            0 => {
                let offset = offset_of!(MIB_IPADDRTABLE, table);
                let count = buffer[0] as usize;
                if count > (size as usize).saturating_sub(offset) / size_of::<MIB_IPADDRROW_XP>() {
                    return Vec::new();
                }
                // The API writes count contiguous rows after the header.
                let rows = unsafe {
                    let first = table.cast::<u8>().add(offset).cast::<MIB_IPADDRROW_XP>();
                    std::slice::from_raw_parts(first, count)
                };
                let mut addresses: Vec<_> = rows.iter()
                    .filter(|row| u32::from(row.wType) & (MIB_IPADDR_DELETED | MIB_IPADDR_DISCONNECTED) == 0)
                    .map(|row| Ipv4Addr::from(row.dwAddr.to_ne_bytes()))
                    .filter(|ip| !ip.is_unspecified() && !ip.is_loopback())
                    .collect();
                addresses.sort_unstable();
                addresses.dedup();
                return addresses;
            }
            _ => break,
        }
    }
    Vec::new()
}

#[cfg(not(windows))]
pub fn local_ipv4_addresses() -> Vec<Ipv4Addr> {
    Vec::new()
}
