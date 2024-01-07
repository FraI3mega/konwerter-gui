#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use konwerter::convert;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Konwerter", options, Box::new(|cc| Box::<MyApp>::default()))
}

struct MyApp {
    number: String,
    mode: Mode,
    base: u32,
}

#[derive(PartialEq)]
enum Mode {
    ToDec,
    ToAlien,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            number: "1234".to_string(),
            mode: Mode::ToDec,
            base: 10,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut result = "Lorem ipsum".to_string();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Konwerter systemów liczbowych");
            ui.horizontal(|ui| {
                let name_label = ui.label("Liczba: ");
                ui.text_edit_singleline(&mut self.number)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.base, 2..=16).text("Podstawa"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, Mode::ToAlien, "Na system obcy");
                ui.selectable_value(&mut self.mode, Mode::ToDec, "Na system dzisiętny");
            });

            if &self.mode == &Mode::ToDec {
                result = format!(
                    "Liczba {} w systemie dzisiętnym to {}",
                    &self.number,
                    convert::to_dec(self.number.clone(), self.base)
                )
            } else {
                result = format!(
                    "Liczba {} w systemie o podstawie {} to {}",
                    &self.number,
                    &self.base,
                    convert::to_alien(self.number.parse().unwrap_or_default(), self.base)
                )
            }

            ui.label(result);
        });
    }
}
