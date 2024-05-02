use std::fs::File;
use std::io::prelude::*;

use crate::guber_state::GuberState;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GuberpunkApp {
    #[serde(skip)]
    device: GuberState,
}

impl Default for GuberpunkApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            device: GuberState::default(),
        }
    }
}

impl GuberpunkApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for GuberpunkApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Guberpunk: Knockoff Chummer");

            ui.horizontal(|ui| {
                ui.label("Controls");
            });
            
            if ui.button("Open File").clicked() {
                let _open_file: String;
                let _file_result: Result<(), String> = match tinyfiledialogs::open_file_dialog("Open", "", None) {
                    None => {
                        _open_file = "null".to_string();
                        Err("No file provided".to_string())
                    }
                    Some(file) => {
                        _open_file = file.clone();
                        let mut handle = File::open(file).expect("Could not open file");
                        let mut rom_buf: Vec<u8> = vec![];
                        handle.read_to_end(&mut rom_buf).expect("Could not read from file");
                        Ok(())
                    }
                };
            }

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.separator();
            });
        });
    }
}

