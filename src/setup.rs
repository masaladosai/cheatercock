use eframe::egui::{self, RichText};
use eframe::{App, Frame};
use std::fs;

#[derive(Default)]
struct SetupApp {
    step: usize,
    api_key: String,
}

impl App for SetupApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.style_mut(|s| {
            s.visuals.window_fill = egui::Color32::BLACK;
            s.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 255, 0));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new(" Setup Cheatercock").size(28.0).color(egui::Color32::from_rgb(0, 255, 0)));
                ui.add_space(20.0);

                match self.step {
                    0 => {
                        ui.label("Welcome to the Setup Wizard!");
                        ui.label("We’ll guide you step by step.");
                        if ui.button("Next ▶").clicked() {
                            self.step += 1;
                        }
                    }
                    1 => {
                        ui.label("Enter your API Key:");
                        ui.text_edit_singleline(&mut self.api_key);
                        
                    if ui.button("Next ▶").clicked() {
                            self.step += 1;
                        }

                    }
                    2 => {
                        ui.label("✅ Setup Complete!");
                        ui.monospace(format!("API Key: {}", mask_api_key(&self.api_key)));
                        
                        if ui.button("💾 Save & Finish").clicked() {
                            let config = format!(
                                "{{  \"api_key\": \"{}\"}}",
                                self.api_key
                            );
                            fs::write("config.json", config).expect("Unable to write file");
                            std::process::exit(0);
                        }
                    }
                    _ => {}
                }
            });
        });
    }
}

fn mask_api_key(s: &str) -> String {
    if s.len() <= 8 {
        "(redacted)".to_string()
    } else {
        let start = &s[..4];
        let end = &s[s.len()-4..];
        format!("{}{}{}", start, "*".repeat(s.len().saturating_sub(8)), end)
    }
}

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "setup cheatercockv1.1",
        options,
        Box::new(|_cc| Box::new(SetupApp::default())),
    )
}
