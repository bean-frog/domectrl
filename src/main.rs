/*
 * domectrl v1.0.0
 * Telescope dome motor controller for Pine Mountain Observatory
 * Controls Robbins & Helio domes over serial port
 * Author: Graeme Kieran
 * License: MIT
 */

mod app;
mod config;
mod protocol;
mod serial;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("domectrl")
            .with_inner_size([480.0, 420.0])
            .with_min_inner_size([480.0, 420.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "domectrl",
        options,
        Box::new(|_cc| Ok(Box::new(app::AppState::new()))),
    )
}
