use eframe::{CreationContext, NativeOptions, egui};
use egui::Ui;

struct MyApp {
    locations: Vec<&'static str>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            locations: vec![
                "Internal Drive",
                "External drive e.g. memory stick, external SSD",
                "SD Card",
                "Network Drive",
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui: &mut Ui| {
            ui.heading("Select a location to search:");

            self.locations.iter().for_each(|location: &&str| {
                if ui.button(*location).clicked() {
                    println!("{} clicked", location);
                    // TODO: navigate to the section for this location
                }
            });
        });
    }
}

fn run_gui() -> eframe::Result<()> {
    eframe::run_native(
        "Diskovery",
        NativeOptions::default(),
        Box::new(|_creation_context: &CreationContext<'_>| Ok(Box::new(MyApp::default()))),
    )
}

fn main() {
    let _ = run_gui();
}
