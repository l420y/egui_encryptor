#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Debug;
use std::ops::Add;

use eframe::{App, NativeOptions, run_native};
use eframe::egui;
use eframe::egui::{Align, Layout};
use rfd;

use Cipher::*;

use crate::encryption::binary_encryption::binary_util;
use crate::encryption::xor_encryption::xor_util;
use crate::Mode::{Decrypt, Encrypt};

pub mod encryption;

const FL: f32 = 10.0;

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(370.0, 170.0)),
        maximized: false,
        vsync: true,
        drag_and_drop_support: true,
        resizable: false,
        fullscreen: false,
        ..eframe::NativeOptions::default()
    };
    run_native("Encryptor", options, Box::new(|_cc| Box::<Enc>::default())).unwrap();
}

#[derive(Debug, PartialEq)]
enum Cipher {
    Xor,
    Binary,
    //Todo()!
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

struct Enc {
    cipher: Cipher,
    mode: Mode,
    picked_path: String,
    display_path: String,
    new_name: String,
    save_as_new: bool,
    key: u8,
}

impl Default for Enc {
    fn default() -> Self {
        Self {
            cipher: Xor,
            mode: Encrypt,
            picked_path: String::default(),
            display_path: String::default(),
            new_name: String::default(),
            save_as_new: false,
            key: 1,
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
                        .selected_text(format!("Cipher: {:?}", self.cipher))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.cipher, Xor, "Xor");
                            ui.selectable_value(&mut self.cipher, Binary, "Binary");
                        });
                    match self.cipher {
                        Xor => {
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
                        Binary => {
                            egui::ComboBox::from_label("  ")
                                .selected_text(format!("Mode: {:?}", &self.mode))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.mode, Encrypt, "Encrypt");
                                    ui.selectable_value(&mut self.mode, Decrypt, "Decrypt");
                                });
                        }
                        _ => {}
                    }
                    ui.end_row();
                });
                ui.add_space(FL);
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    if ui.button("Open Explorer...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = path.to_str().unwrap().to_string();
                            self.display_path = self.picked_path.clone();
                            self.new_name = self.picked_path.clone();
                            if self.display_path.chars().count() > 33 {
                                self.display_path = self.picked_path.clone()[0..30].to_string().add("..");
                            }
                        }
                    }
                    ui.add_space(FL);
                    ui.horizontal(|ui| {
                        ui.monospace(&self.display_path);
                        ui.add_space(FL);
                    });
                });
            });
            ui.add_space(FL);
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.checkbox(&mut self.save_as_new, "Save as new");
                ui.add_space(FL);
                if self.save_as_new {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.new_name);
                    });
                }
            });
            ui.add_space(FL);
            if ui.button("Start").clicked() && self.picked_path != String::default() {
                if !self.save_as_new {
                    self.new_name = String::from(&self.picked_path);
                }
                match self.cipher {
                    Xor => {
                        xor_util(&self.picked_path, &self.new_name, self.key);
                    }
                    Binary => {
                        match self.mode {
                            Encrypt => {
                                binary_util(&self.picked_path, &self.new_name, false);
                            }
                            Decrypt => {
                                binary_util(&self.picked_path, &self.new_name, true);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}