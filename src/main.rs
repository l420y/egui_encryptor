use std::fmt::Debug;

use eframe::{App, NativeOptions, run_native};
use eframe::egui;
use eframe::egui::{Align, Layout};
use rfd;

use Mode::*;

use crate::encryption::binary_encryption::binary_util;
use crate::encryption::xor_encryption::xor_util;

pub mod encryption;

const FL: f32 = 10.0;

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        drag_and_drop_support: true,
        resizable: false,
        fullscreen: false,
        ..eframe::NativeOptions::default()
    };
    run_native("Encryptor", options, Box::new(|_cc| Box::<Enc>::default())).unwrap();
}

#[derive(Debug, PartialEq)]
enum Mode {
    Xor,
    Binary,
    //Todo()!
}

struct Enc {
    mode: Mode,
    key: u8,
    picked_path: String,
    new_name: String,
    save_as_new: bool,
}

impl Default for Enc {
    fn default() -> Self {
        Self {
            mode: Xor,
            key: 1,
            picked_path: String::default(),
            new_name: String::default(),
            save_as_new: false,
        }
    }
}

impl App for Enc {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.label(egui::RichText::new("Settings:").size(FL * 2.0));
                ui.add_space(FL);
                egui::Grid::new("mode_grid").show(ui, |ui| {
                    egui::ComboBox::from_label(String::new())
                        .selected_text(format!("Mode: {:?}", self.mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.mode, Xor, "Xor");
                            ui.selectable_value(&mut self.mode, Binary, "Binary");
                            //Todo()!
                        });

                    if self.mode == Xor {
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_label(" ")
                                .selected_text(format!("Key: {:?}", self.key))
                                .show_ui(ui, |ui| {
                                    for key in 1..=255 {
                                        ui.selectable_value(&mut self.key, key, key.to_string());
                                    }
                                });
                        });
                    }
                    ui.end_row();
                });
                ui.add_space(FL);
            });

            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                if ui.button("Browse Explorer...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = path.display().to_string();
                    }
                }
            });

            ui.add_space(FL);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("File:").size(FL * 1.5));
                ui.monospace(&self.picked_path);
                ui.add_space(FL);
            });

            ui.add_space(FL);
            ui.checkbox(&mut self.save_as_new, "Save as new file");
            ui.add_space(FL);
            if self.save_as_new {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("New file:").size(FL * 1.5));
                    ui.add_space(FL);
                    ui.text_edit_singleline(&mut self.new_name);
                    ui.add_space(FL);
                });
            }

            if ui.button("Start").clicked() && self.picked_path != String::default() {
                if !self.save_as_new {
                    self.new_name = String::from(&self.picked_path);
                }

                match self.mode {
                    Xor => {
                        xor_util(&self.picked_path, &self.new_name, self.key);
                    }

                    Binary => {
                        binary_util(&self.picked_path, &self.new_name);
                    }
                    _ => {} //Todo()!
                }
            }
        });
    }
}