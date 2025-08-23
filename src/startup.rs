use eframe::egui::{self, RichText};
use egui::vec2;
use std::thread;
use std::time::Duration;
use egui::ColorImage;
use egui_extras::RetainedImage;
use std::fs;
use std::process::Command;




const MAX_CONSOLE_LINES: usize = 100;
pub struct MainApp {
    selected_language: String,
    console_lines: Vec<String>,
    last_update_time: f64,
    image: RetainedImage,
    is_running: bool,
    current_lang: String,
}

impl Default for MainApp {
    fn default() -> Self {
        let image_bytes = fs::read("assets/cheatercock.png").expect("Failed to read image");
        let image = RetainedImage::from_image_bytes("cheatercock.png", &image_bytes).unwrap();
        Self {
            selected_language: "python".to_string(),
            console_lines: vec!["[SYSTEM_INIT] Console ready.".to_string()],
            last_update_time: 0.0,
            is_running: false,
            image,
            current_lang:String::new(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

       
        ctx.request_repaint();
        ctx.style_mut(|s| {
            s.visuals.window_fill = egui::Color32::BLACK;
            s.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 255, 0));
        });
        if self.is_running {
            egui::TopBottomPanel::bottom("hacker_console")
                .resizable(true) 
                .default_height(150.0)
                .min_height(50.0)
                .show(ctx, |ui| {
                    ui.add_space(5.0);


                    let Store:Vec<String> = vec![ "[App Booted] Cheatercock is starting...".to_string(),

"[Reading Api_Key] CC trying to read api_key...".to_string(),

"[Enabling Hotkeys] Listing to keyboard...".to_string(),

"<HOTKEY to ADD> ctrl + leftshift + alt + k".to_string(),

 "<HOTKEY to PROMPT> ctrl + leftshift + alt + p".to_string(),

"<HOTKEY to EXIT> ctrl + leftshift + alt + q".to_string()];
                    egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                        for line in Store {
                            ui.monospace(line);
                        }
                            for line in &self.console_lines {
        ui.monospace(line);
    }
                    });
                    ui.add_space(5.0);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {

                ui.add_space(20.0);

                ui.heading(
                    RichText::new("Start Cheatercock")
                        .size(24.0)
                        .color(egui::Color32::from_rgb(0, 255, 0)),
                );

                ui.add_space(100.0);
  
            ui.horizontal_wrapped(|ui|{
            let total_width = ui.available_width();
            let widget_width = 200.0; 
            let left_margin = (total_width - widget_width) / 2.0;
            ui.add_space(left_margin);
                egui::ComboBox::from_id_source("language_combo_string") 
                    .selected_text(
            RichText::new(&self.selected_language).size(25.0)
        )
                    .show_ui(ui, |ui| {
                         ui.selectable_value(&mut self.selected_language, "cpp".to_string(), "C++");
                        ui.selectable_value(&mut self.selected_language, "rust".to_string(), "Rust");
                        ui.selectable_value(&mut self.selected_language, "python".to_string(), "Python");
                        ui.selectable_value(&mut self.selected_language, "java".to_string(), "Java");
                       
                        ui.selectable_value(&mut self.selected_language, "go".to_string(), "GoLang");
                        ui.selectable_value(&mut self.selected_language, "bash".to_string(), "Bash");
                        ui.selectable_value(&mut self.selected_language, "sql".to_string(), "SQL");
                        ui.selectable_value(
                            &mut self.selected_language,
                            "javascript".to_string(),
                            "JavaScript",
                        );
                    });
                     if ui.add(egui::Button::new("Run").min_size(vec2(140.0, 30.0))).clicked() {



                        if !self.is_running {
                             self.console_lines.push(format!("[STARTING] Starting Cheatercock with language:{}",self.selected_language));
                             self.current_lang= self.selected_language.clone();
                            println!("starting with language:{}", self.selected_language);
                            let mut prompt = String::new();
                            if self.selected_language == "python" {
                                prompt = "you have to generate code for python for the given question...it might include some precoded snipped and test cases as well use ur logic and dont use unnecessary comments".to_string();
                                
                            } else if self.selected_language == "java" {
                             prompt = "you have to generate code for java for the given question...it might include some precoded snipped and test cases as well use ur logic and dont use unnecessary comments".to_string();
                            } else if self.selected_language == "cpp" {
                                prompt = "you have to generate code for cpp for the given question,it might be related to dsa..it might include some precoded snipped and test cases as well use ur logic and dont use unnecessary comments".to_string();
                            } else if self.selected_language == "javascript" {
                                prompt = "you have to generate code for python for the given question...it might include some precoded snipped and test cases as well use ur logic and dont use unnecessary comments".to_string();
                            } else if self.selected_language == "rust" {
                                prompt = "you have to generate code for rust for the given question".to_string();
                            } else if self.selected_language == "go" {
                                prompt = "you have to generate code for golang for the given question".to_string();

                            } else if self.selected_language == "bash" {
                                prompt = "you have to generate bash,shell,sed,awk etc for linux for the given question it include question along with test cases..use comments for instructions".to_string();
                            }else if self.selected_language == "sql" {
                                prompt = "you have to generate code for postgressql only if all the data is given for the question for the given question...it might include testcases..if the text dont have sufficient table data required to execute question ask them about it and dont generate code then".to_string();
                            } else {
                                prompt="solve the given question with code".to_string();
                            }
                             let mut child = Command::new("cargo")
                            .arg("run")
                            .arg("--bin")
                            .arg("logic")
                            .arg("--release")
                            .arg(prompt) 
                            .spawn()
                            .expect("failed to run logic.rs");

                        println!("Launched logic.rs with PID: {:?}", child.id());
                            
                        self.is_running = true;
                        } else {
                            println!("Cheatercock is already running.");
                            let data:String = format!("<Program in Process>cheatercock is already running with:{}\n<notice:>if not preferred language press ctrl+lshift+alt+q and then restart app to select new languge",self.current_lang);
                            self.console_lines.push(data);
                        }
                      
                    }  
                    });
                

               self.image.show_max_size(ui, vec2(128.0, 128.0));
               
             
            
        });
        });


         
    }
}

// Function for main.rs to call
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Startup App",
        options,
Box::new(|_cc| Box::new(MainApp::default())),
    )
}

