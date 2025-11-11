use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let icon_data = eframe::icon_data::from_png_bytes(
            include_bytes!("../resources/dti.png")
        )
        .expect("Failed to load icon");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            icon: Some(Arc::new(icon_data)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Doble Technologies - Chat Application",
        options,
        Box::new(|_cc| Ok(Box::new(ChatApp {})))
    )
}

struct ChatApp;

impl eframe::App for ChatApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}