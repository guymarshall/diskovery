use eframe::{CreationContext, NativeOptions, egui};
use egui::{Button, Ui, vec2};

enum Screen {
    Locations,
    InternalDrives,
}

struct MyApp {
    screen: Screen,
    locations: Vec<&'static str>,
    internal_drives: Vec<&'static str>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screen: Screen::Locations,
            locations: vec![
                "Internal Drive",
                "External drive e.g. memory stick, external SSD",
                "SD Card",
                "Network Drive",
            ],
            // TODO: replace with real drive detection
            internal_drives: vec!["/dev/sda", "/dev/nvme0n1"],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Locations => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select a location to search:");

                    let available_height: f32 = ui.available_height();
                    let button_count: usize = self.locations.len();
                    let padding: f32 = 16.0;
                    let button_spacing: f32 = 12.0;
                    let total_spacing: f32 =
                        padding * 2.0 + button_spacing * (button_count as f32 - 1.0);
                    let button_height: f32 =
                        ((available_height - total_spacing) / button_count as f32).max(32.0);

                    ui.add_space(padding);

                    self.locations
                        .iter()
                        .enumerate()
                        .for_each(|(i, location): (usize, &&str)| {
                            let button: Button<'_> = Button::new(*location)
                                .min_size(vec2(ui.available_width(), button_height));
                            if ui.add(button).clicked() {
                                if *location == "Internal Drive" {
                                    self.screen = Screen::InternalDrives;
                                } else {
                                    println!("{} clicked", location);
                                    // TODO: handle other locations
                                }
                            }
                            if i < button_count - 1 {
                                ui.add_space(button_spacing);
                            }
                        });

                    ui.add_space(padding);
                });
            }
            Screen::InternalDrives => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select an internal drive:");
                    ui.add_space(16.0);

                    self.internal_drives.iter().enumerate().for_each(
                        |(i, drive): (usize, &&str)| {
                            let button =
                                Button::new(*drive).min_size(vec2(ui.available_width(), 40.0));
                            if ui.add(button).clicked() {
                                println!("{} clicked", drive);
                            }
                            if i < self.internal_drives.len() - 1 {
                                ui.add_space(8.0);
                            }
                        },
                    );

                    ui.add_space(16.0);
                    if ui.add(Button::new("Back")).clicked() {
                        self.screen = Screen::Locations;
                    }
                });
            }
        }
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
