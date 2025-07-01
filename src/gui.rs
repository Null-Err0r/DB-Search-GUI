use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::search;
use rfd::FileDialog;
use std::thread; 


enum SearchStatus {
    Idle,
    InProgress(String),
    Completed(String),  
    Error(String), 
}

pub struct SearchApp {
    folder_path: String,
    pattern: String,
    results: Arc<Vec<String>>,
    current_status: SearchStatus, 

    search_result_tx: Arc<Mutex<Option<Result<String, String>>>>,
}

impl Default for SearchApp {
    fn default() -> Self {
        Self {
            folder_path: String::new(),
            pattern: String::new(),
            results: Arc::new(vec![]),
            current_status: SearchStatus::Idle,
            search_result_tx: Arc::new(Mutex::new(None)),
        }
    }
}

impl eframe::App for SearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(result) = self.search_result_tx.lock().unwrap().take() {
            match result {
                Ok(msg) => self.current_status = SearchStatus::Completed(msg),
                Err(msg) => self.current_status = SearchStatus::Error(msg),
            }
       
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📁 Folder path and search pattern ");

            if ui.button("📂 select folder...").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.folder_path = path.display().to_string();
                }
            }
            ui.label(format!("Selected folder: {}", self.folder_path));

            ui.horizontal(|ui| {
                ui.label("🔍 Searchable text:");
                ui.text_edit_singleline(&mut self.pattern);
            });


            let is_search_active = matches!(self.current_status, SearchStatus::InProgress(_));
            if ui.add_enabled(!is_search_active, egui::Button::new("🚀 Start search")).clicked() {
                let folder = self.folder_path.clone();
                let pat = self.pattern.clone();
                let ctx_clone = ctx.clone();
                let result_tx_clone = Arc::clone(&self.search_result_tx);

                self.current_status = SearchStatus::InProgress("Searching 🔍 Please wait ...".to_string());
                self.results = Arc::new(vec![]); 

                thread::spawn(move || {
                    let output_path = PathBuf::from("results.txt");
                    let search_result = match search::search_folder(&PathBuf::from(folder), &pat, Some(&output_path)) {
                        Ok(total) => {
                            Ok(format!("✅ Number of lines matched: {} (results saved in results.txt). \n\nSearch completed.", total))
                        },
                        Err(e) => {
                            Err(format!("❌ Error: {}. Search ended.", e))
                        },
                    };

               
                    *result_tx_clone.lock().unwrap() = Some(search_result);
                    ctx_clone.request_repaint(); 
                });
            }

            ui.separator();

            match &self.current_status {
                SearchStatus::Idle => {
                    ui.label("Ready to search.");
                }
                SearchStatus::InProgress(msg) => {
                    ui.label(msg);
                }
                SearchStatus::Completed(msg) | SearchStatus::Error(msg) => {
                    ui.label(msg);
                }
            }

            if let SearchStatus::Completed(_) = self.current_status {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for line in self.results.iter() {
                        ui.label(line);
                    }
                });
            }
        });
    }
}
