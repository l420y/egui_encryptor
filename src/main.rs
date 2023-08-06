#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{NativeOptions, run_native};
use eframe::egui;

use ui::Enc;

pub mod encryption;
pub mod ui;

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 400.0)),
        maximized: false,
        vsync: true,
        drag_and_drop_support: true,
        resizable: false,
        fullscreen: false,
        ..eframe::NativeOptions::default()
    };
    run_native("Encryptor", options, Box::new(|_cc| Box::<Enc>::default())).unwrap();
}
