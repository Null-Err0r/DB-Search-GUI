use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::search;
use rfd::FileDialog;
use std::thread;

use unicode_bidi::BidiInfo;
use arabic_reshaper::ArabicReshaper;

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
        setup_fonts(ctx);

        if let Some(result) = self.search_result_tx.lock().unwrap().take() {
            match result {
                Ok(msg) => self.current_status = SearchStatus::Completed(fix_farsi_text(&msg)),
                Err(msg) => self.current_status = SearchStatus::Error(fix_farsi_text(&msg)),
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(fix_farsi_text("📁 مسیر پوشه و الگوی جستجو"));

            if ui.button(fix_farsi_text("📂 انتخاب پوشه...")).clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.folder_path = path.display().to_string();
                }
            }

            ui.label(fix_farsi_text(&format!("📂 پوشه انتخاب شده: {}", self.folder_path)));

            ui.horizontal(|ui| {
                ui.label(fix_farsi_text("🔍 متن قابل جستجو:"));

                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    use egui::{FontId, Color32};
                    let job = egui::text::LayoutJob::simple(
                        fix_farsi_text(string),
                        FontId::monospace(16.0),
                        Color32::WHITE,
                        wrap_width,
                    );
                    ui.fonts(|f| f.layout_job(job))
                };

                ui.add(
                    egui::TextEdit::singleline(&mut self.pattern)
                        .layouter(&mut layouter)
                );
            });

            let is_search_active = matches!(self.current_status, SearchStatus::InProgress(_));
            if ui.add_enabled(!is_search_active, egui::Button::new(fix_farsi_text("🚀 شروع جستجو"))).clicked() {
                let folder = self.folder_path.clone();
                let pat = self.pattern.clone();
                let ctx_clone = ctx.clone();
                let result_tx_clone = Arc::clone(&self.search_result_tx);

                self.current_status = SearchStatus::InProgress(fix_farsi_text("در حال جستجو... لطفا صبر کنید ⏳"));
                self.results = Arc::new(vec![]);

                thread::spawn(move || {
                    let output_path = PathBuf::from("results.txt");
                    let search_result = match search::search_folder(&PathBuf::from(folder), &pat, Some(&output_path)) {
                        Ok(total) => Ok(format!("✅ تعداد خطوط مطابق: {} (نتایج در results.txt ذخیره شدند).", total)),
                        Err(e) => Err(format!("❌ خطا: {}.", e)),
                    };

                    *result_tx_clone.lock().unwrap() = Some(search_result);
                    ctx_clone.request_repaint();
                });
            }

            ui.separator();

            match &self.current_status {
                SearchStatus::Idle => {
                    ui.label(fix_farsi_text("🟢 آماده برای جستجو"));
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
                        ui.label(fix_farsi_text(line));
                    }
                });
            }
        });
    }
}


fn fix_farsi_text(input: &str) -> String {
    let reshaper = ArabicReshaper::default();
    let reshaped = reshaper.reshape(input);

    let bidi_info = BidiInfo::new(&reshaped, None);

    if bidi_info.paragraphs.is_empty() {
        return input.to_string(); 
    }

    let para = &bidi_info.paragraphs[0];
    bidi_info.reorder_line(para, 0..reshaped.len()).to_string()
}



fn setup_fonts(ctx: &egui::Context) {
    use egui::FontFamily::{Monospace, Proportional};

    let vazir_font = std::fs::read("/usr/share/fonts/truetype/vazir/farsi.ttf")
        .expect("❌ فونت Vazir پیدا نشد. مسیر را بررسی کن.");

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Vazir".to_owned(),
        egui::FontData::from_owned(vazir_font),
    );

    fonts.families.entry(Proportional).or_default().insert(0, "Vazir".to_owned());
    fonts.families.entry(Monospace).or_default().insert(0, "Vazir".to_owned());

    ctx.set_fonts(fonts);
}
