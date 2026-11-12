use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows")
        || env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc")
    {
        return;
    }
    println!("cargo:rerun-if-changed=assets/printer.rc");
    println!("cargo:rerun-if-changed=assets/printer.ico");

    let sdk_bin = PathBuf::from(env::var_os("ProgramFiles(x86)").unwrap_or_default())
        .join("Windows Kits/10/bin");
    let compiler = fs::read_dir(sdk_bin)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("x64/rc.exe"))
        .filter(|path| path.is_file())
        .max()
        .unwrap_or_else(|| PathBuf::from("rc.exe"));
    let resource = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo OUT_DIR")).join("printer.res");
    let status = Command::new(compiler)
        .current_dir("assets")
        .arg("/nologo")
        .arg("/fo")
        .arg(&resource)
        .arg("printer.rc")
        .status()
        .expect("run Windows SDK resource compiler");
    assert!(status.success(), "could not embed the printer icon");
    println!("cargo:rustc-link-arg-bin=escpos_emulator={}", resource.display());
}
