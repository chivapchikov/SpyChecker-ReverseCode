use eframe::egui::{self, CentralPanel};
use eframe::egui::{Color32, Frame};


#[derive(Default)]
pub struct App {
    // Здесь позже будут поля
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let my_frame = Frame::default()
         .fill(Color32::from_rgb(18, 20, 26));

     CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            ui.heading("SpyChecker - ReverseCode");
        }); 
    }
}


pub fn run_gui() -> eframe::Result<()> {

    let options_eframe = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "SpyChecker",
        options_eframe,
        Box::new(|_cc| Box::new(App::default())),
    )
}