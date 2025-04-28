#![windows_subsystem = "windows"]
/// ./src/main.rs

use calculator_frontend::CalcGUI;
use eframe::egui::{Vec2, ViewportBuilder};
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2::new(400.0, 600.0)) // Set the fixed window size
            .with_resizable(false),                  // Make the window non-resizable
        ..Default::default()
    };

    eframe::run_native(
        "🔥Calculator🔥",
        options,
        Box::new(|_cc| Ok(Box::new(CalcGUI::new()))),
    )
}