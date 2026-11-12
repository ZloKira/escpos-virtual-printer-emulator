#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use escpos_emulator::emulator::EmulatorState;
use escpos_emulator::gui::EscPosEmulatorApp;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting ESC/POS Emulator...");

    let emulator_state = Arc::new(Mutex::new(EmulatorState::new()));

    let tokio_handle = tokio::runtime::Handle::current();

    info!("✅ Emulator initialized successfully");

    let icon = image::load_from_memory(include_bytes!("../assets/printer.png"))
        .expect("embedded printer icon")
        .into_rgba8();
    let (width, height) = icon.dimensions();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(egui::IconData {
            rgba: icon.into_raw(),
            width,
            height,
        }),
        ..Default::default()
    };

    let result = eframe::run_native(
        "ESC/POS Virtual Printer Emulator - fork by KiraZlo",
        options,
        Box::new(move |_cc| Box::new(EscPosEmulatorApp::new(emulator_state, tokio_handle))),
    );
    if let Err(error) = &result {
        let path = std::env::temp_dir().join("ESCPosVirtualPrinter-startup.log");
        let message = format!("GUI startup failed: {error}\n{error:?}\n");
        if let Err(log_error) = std::fs::write(path, &message) {
            eprintln!("{message}Could not write startup log: {log_error}");
        }
    }
    result
}
