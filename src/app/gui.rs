use anyhow::{Ok, Result};
use eframe::egui;
use std::{fmt::format, sync::Arc};

pub struct App {
    pub app_name: String,
    pub connection_status: bool,
}

pub trait Dongle {
    fn open() -> Result<()>;
    fn close() -> Result<()>;
}

impl Default for App {
    fn default() -> Self {
        App {
            app_name: "SDR".to_owned(),
            connection_status: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.app_name.clone());
            if ui.button("Connect").clicked() {
                self.connection_status = !self.connection_status;
            }
            ui.label(format!("connected: {}", self.connection_status));
        });
    }
}

impl App {
    pub fn display_window() -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };
        eframe::run_native(
            Default::default(),
            options,
            Box::new(|cc| std::result::Result::Ok(Box::<App>::default())),
        )
    }
}
