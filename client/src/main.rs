use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

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