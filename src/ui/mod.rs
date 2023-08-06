#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Debug;
use std::fs::File;
use std::ops::Add;
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use eframe::App;
use eframe::egui;
use eframe::egui::{Align, Color32, Layout};
use rfd;

use Cipher::*;
use Mode::{Decrypt, Encrypt};

use crate::encryption::base64_encryption::base64_util;
use crate::encryption::binary_encryption::binary_util;
use crate::encryption::xor_encryption::xor_util;

const FL: f32 = 10.0;

#[derive(Debug, PartialEq)]
enum Cipher {
    Xor,
    Binary,
    Base64,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct Enc {
    cipher: Cipher,
    mode: Mode,
    picked_path: String,
    display_path: String,
    new_name: String,
    file_extension: String,
    save_as_new: bool,
    readonly: bool,
    key: u8,
    file_size: u64,
}

impl Default for Enc {
    fn default() -> Self {
        Self {
            cipher: Xor,
            mode: Encrypt,
            picked_path: String::default(),
            display_path: String::default(),
            new_name: String::default(),
            file_extension: String::default(),
            save_as_new: false,
            readonly: false,
            key: 1,
            file_size: u64::default(),
        }
    }
}

impl App for Enc {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("mode_grid").show(ui, |ui| {
                ui.label(egui::RichText::new("Settings").size(FL * 2.5));
                ui.add_space(FL * 3.0);
                ui.label(egui::RichText::new("Info").size(FL * 2.5));
                ui.end_row();
                egui::ComboBox::from_label(String::new())
                    .selected_text(format!("Cipher: {:?}", self.cipher))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.cipher, Xor, "Xor");
                        ui.selectable_value(&mut self.cipher, Binary, "Binary");
                        ui.selectable_value(&mut self.cipher, Base64, "Base64");
                    });
                ui.add_space(FL * 3.0);
                ui.label(egui::RichText::new("File path:").size(FL * 1.5));
                ui.label(egui::RichText::new(&self.display_path).color(Color32::DARK_GREEN));
                ui.end_row();
                match self.cipher {
                    Xor => {
                        egui::ComboBox::from_label(" ")
                            .selected_text(format!("Key: {:?}", self.key))
                            .show_ui(ui, |ui| {
                                for key in 1..=255 {
                                    ui.selectable_value(&mut self.key, key, key.to_string());
                                }
                            });
                    }
                    _ => {
                        egui::ComboBox::from_label("  ")
                            .selected_text(format!("Mode: {:?}", &self.mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.mode, Encrypt, "Encrypt");
                                ui.selectable_value(&mut self.mode, Decrypt, "Decrypt");
                            });
                    }
                }

                ui.add_space(FL * 3.0);
                ui.label(egui::RichText::new("File size:").size(FL * 1.5));
                if self.file_size != 0 { ui.label(egui::RichText::new(&self.file_size.to_string().add(" bytes")).color(Color32::DARK_GREEN)); }
                ui.end_row();
                if ui.button("Open Explorer...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = path.to_str().unwrap().to_string();
                        self.display_path = self.picked_path.clone();
                        self.new_name = self.picked_path.clone();
                        self.file_size = std::fs::metadata(&self.picked_path).unwrap().file_size();
                        self.file_extension = Path::new(&self.picked_path).extension().unwrap().to_str().unwrap().to_string().replace('"', "");
                        if self.display_path.chars().count() > 99 {
                            self.display_path = self.picked_path.clone()[0..96].to_string().add("..");
                        }
                    }
                }

                ui.add_space(FL * 3.0);
                ui.label(egui::RichText::new("File type:").size(FL * 1.5));
                ui.label(egui::RichText::new(&self.file_extension.to_uppercase()).color(Color32::DARK_GREEN));
                ui.end_row();
                ui.checkbox(&mut self.save_as_new, "Save as new");
                ui.add_space(FL * 3.0);
                ui.label(egui::RichText::new("Readonly:").size(FL * 1.5));
                if self.picked_path != String::default() { ui.label(egui::RichText::new(&self.readonly.to_string()).color(Color32::DARK_GREEN)); }
                ui.end_row();
                if self.save_as_new {
                    ui.text_edit_singleline(&mut self.new_name);
                    ui.end_row();
                }
                ui.checkbox(&mut self.readonly, "Make readonly");
                if self.readonly && self.picked_path != String::default() { File::open(&self.picked_path).unwrap().metadata().unwrap().permissions().set_readonly(self.readonly); }
            });
            //Grid end
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
                        if ui.button("Start process").clicked() && self.picked_path != String::default() {
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
                                    }
                                }
                                Base64 => {
                                    match self.mode {
                                        Encrypt => {
                                            base64_util(&self.picked_path, &self.new_name, false);
                                        }
                                        Decrypt => {
                                            base64_util(&self.picked_path, &self.new_name, true);
                                        }
                                    }
                                }
                            }
                        }
                    });
                    ui.end_row();
                });
            });
        });
    }
}