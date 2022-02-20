mod memefinder;

use eframe::{
    NativeOptions, 
    run_native, 
    epi::App, egui::{CentralPanel, ScrollArea}
};

use memefinder::MemeFinder;

impl App for MemeFinder {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
       CentralPanel::default().show(ctx, |ui|{
            ScrollArea::vertical().stick_to_right().show(ui, |ui| {   
                self.load(ui); 
                self.load_buttons(&ctx, ui); 
            });
       });
    
       self.save_data();
    }

    fn name(&self) -> &str {
        "MemeFinder"
    }

    fn setup(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        self.add_font(&ctx);
    }
}

fn main() {
    let app = MemeFinder::new();
    let mut win_option = NativeOptions::default();

    win_option.maximized = true;

    run_native(Box::new(app), win_option);
}

