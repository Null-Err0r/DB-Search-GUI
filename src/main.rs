

mod search;
mod gui;

use gui::SearchApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
    "DB Search",
        options,
        Box::new(|_cc| Box::new(SearchApp::default())),
    )
}

