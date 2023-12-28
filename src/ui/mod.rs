#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Debug;
use std::fs::File;
use std::ops::Add;
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use eframe::App;
use eframe::egui;
use eframe::egui::{Align, Align2, Color32, FontId, Layout, Pos2, Rect, Sense, Stroke, Vec2};
use rfd;

use Cipher::*;
use Mode::{Decrypt, Encrypt};

use crate::encryption::base64_encryption::base64_util;
use crate::encryption::binary_encryption::binary_util;
use crate::encryption::xor_encryption::xor_util;

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

pub struct EncryptorApp {
    cipher: Cipher,
    mode: Mode,
    picked_path: String,
    display_path: String,
    new_name: String,
    file_extension: String,
    save_as_new: bool,
    readonly: bool,
    file_size: u64,
    key: u8,
}

impl Default for EncryptorApp {
    fn default() -> Self {
        Self {
            cipher: Xor,
            mode: Encrypt,
            key: 1,
            picked_path: String::default(),
            display_path: String::default(),
            new_name: String::default(),
            file_extension: String::default(),
            file_size: u64::default(),
            save_as_new: false,
            readonly: false,
        }
    }
}

impl App for EncryptorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.colored_label(Color32::WHITE, "Encryptor");
            });
            ui.separator();
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::top_down(Align::TOP), |ui| {
                    ui.label(egui::RichText::new("Settings:  ").size(25.0).color(Color32::LIGHT_GRAY));
                    ui.add_space(5.0);
                    egui::ComboBox::from_label("").selected_text(format!("Cipher: {:?}", self.cipher))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.cipher, Xor, "Xor");
                            ui.selectable_value(&mut self.cipher, Binary, "Binary");
                            ui.selectable_value(&mut self.cipher, Base64, "Base64");
                        });
                    ui.add_space(3.5);
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

                    ui.add_space(3.5);
                    if ui.button("Open Explorer...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = path.to_str().unwrap().to_string();
                            self.display_path = self.picked_path.clone();
                            self.new_name = self.picked_path.clone();
                            self.file_size = std::fs::metadata(&self.picked_path).unwrap().file_size();
                            self.file_extension = Path::new(&self.picked_path).extension().unwrap().to_str().unwrap().to_string();
                        }
                    }
                    ui.add_space(3.5);
                    ui.checkbox(&mut self.save_as_new, "Save as new");
                    if self.save_as_new {
                        ui.text_edit_singleline(&mut self.new_name);
                    }
                    ui.add_space(3.5);
                    ui.checkbox(&mut self.readonly, "Make readonly");
                    if self.readonly && self.picked_path != String::default() { File::open(&self.picked_path).unwrap().metadata().unwrap().permissions().set_readonly(self.readonly); }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.label(egui::RichText::new("File info:  ").size(25.0).color(Color32::LIGHT_GRAY));
                    ui.label(egui::RichText::from(format!("File path: {}", &self.display_path)).size(15.0));
                    ui.label(egui::RichText::new(format!("File size: {}", &self.file_size.to_string().add(" b"))).size(15.0));
                    ui.label(egui::RichText::new(format!("File type: {}", &self.file_extension)).size(15.0));
                    ui.label(egui::RichText::new(format!("File readonly: {}", &self.readonly.to_string())).size(15.0));
                });
            });
            let start_button: Rect = egui::Rect::from_center_size(Pos2::new(55.0, 230.0), Vec2::new(100.0, 30.0));
            ui.painter().rect(start_button, 30.0, Color32::DARK_GRAY, Stroke::default());
            ui.painter().text(Pos2::new(55.0, 230.0), Align2::CENTER_CENTER, "Start", FontId::default(), Color32::LIGHT_GRAY);
            if ui.allocate_rect(start_button, Sense::click()).clicked() && self.picked_path != String::default() {
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
    }
}