#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{NativeOptions, run_native};
use eframe::egui;

use ui::EncryptorApp;

pub mod encryption;
pub mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 250.0]).with_fullscreen(true).with_always_on_top(),
        centered: true,
        vsync: false,
        ..Default::default()
    };
    run_native("Encryptor", options, Box::new(|cc| { Box::<EncryptorApp>::default() }),
    )
}