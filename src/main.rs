use std::process::exit;

use eframe::{CreationContext, NativeOptions, egui};
use egui::{Button, Ui, vec2};

// const UNFINISHED_LOCATIONS: [&str; 4] = [
//     "Internal Drives",
//     "External Drives",
//     "SD Cards",
//     "Network Drives",
// ];

const UNFINISHED_LOCATIONS: [&str; 0] = [];

enum Screen {
    Locations,
    InternalDrives,
    ExternalDrives,
    SdCard,
    NetworkDrive,
}

struct MyApp {
    screen: Screen,
    locations: Vec<&'static str>,
    internal_drives: Vec<&'static str>,
    external_drives: Vec<&'static str>,
    sd_cards: Vec<&'static str>,
    network_drives: Vec<&'static str>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screen: Screen::Locations,
            locations: vec![
                "Internal Drives",
                "External Drives",
                "SD Cards",
                "Network Drives",
            ],
            // TODO: replace with real drive detection
            internal_drives: vec!["/dev/sda", "/dev/nvme0n1"],
            external_drives: vec!["/dev/sdb", "/dev/sdc"],
            sd_cards: vec!["/dev/mmcblk0", "/dev/mmcblk1"],
            network_drives: vec!["//network/share1", "//network/share2"],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let padding: f32 = 16.0;

        match self.screen {
            Screen::Locations => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select a location to search:");

                    let available_height: f32 = ui.available_height();
                    let button_count: usize = self.locations.len();
                    let button_spacing: f32 = 12.0;
                    let total_spacing: f32 =
                        padding * 2.0 + button_spacing * (button_count as f32 - 1.0);
                    let button_height: f32 =
                        ((available_height - total_spacing) / button_count as f32).max(32.0);

                    ui.add_space(padding);

                    if ui.add(Button::new("Exit")).clicked() {
                        exit(0);
                    }

                    ui.add_space(padding);

                    self.locations
                        .iter()
                        .enumerate()
                        .for_each(|(i, location): (usize, &&str)| {
                            let button: Button<'_> = Button::new(*location)
                                .min_size(vec2(ui.available_width(), button_height));

                            let is_enabled: bool = !UNFINISHED_LOCATIONS.contains(location);

                            if ui.add_enabled(is_enabled, button).clicked() {
                                match *location {
                                    "Internal Drives" => self.screen = Screen::InternalDrives,
                                    "External Drives" => self.screen = Screen::ExternalDrives,
                                    "SD Cards" => self.screen = Screen::SdCard,
                                    "Network Drives" => self.screen = Screen::NetworkDrive,
                                    _ => panic!("Unknown location: {}", location),
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
                    ui.add_space(padding);

                    if ui.add(Button::new("Back")).clicked() {
                        self.screen = Screen::Locations;
                    }

                    ui.add_space(padding);

                    self.internal_drives.iter().enumerate().for_each(
                        |(i, drive): (usize, &&str)| {
                            let button: Button<'_> =
                                Button::new(*drive).min_size(vec2(ui.available_width(), 40.0));
                            if ui.add(button).clicked() {
                                println!("{} clicked", drive);
                            }
                            if i < self.internal_drives.len() - 1 {
                                ui.add_space(8.0);
                            }
                        },
                    );
                });
            }
            Screen::ExternalDrives => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select an external drive:");
                    ui.add_space(padding);

                    if ui.add(Button::new("Back")).clicked() {
                        self.screen = Screen::Locations;
                    }

                    ui.add_space(padding);

                    self.external_drives.iter().enumerate().for_each(
                        |(i, drive): (usize, &&str)| {
                            let button: Button<'_> =
                                Button::new(*drive).min_size(vec2(ui.available_width(), 40.0));
                            if ui.add(button).clicked() {
                                println!("{} clicked", drive);
                            }
                            if i < self.external_drives.len() - 1 {
                                ui.add_space(8.0);
                            }
                        },
                    );
                });
            }
            Screen::SdCard => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select an SD Card:");
                    ui.add_space(padding);

                    if ui.add(Button::new("Back")).clicked() {
                        self.screen = Screen::Locations;
                    }

                    ui.add_space(padding);

                    self.sd_cards
                        .iter()
                        .enumerate()
                        .for_each(|(i, card): (usize, &&str)| {
                            let button: Button<'_> =
                                Button::new(*card).min_size(vec2(ui.available_width(), 40.0));
                            if ui.add(button).clicked() {
                                println!("{} clicked", card);
                            }
                            if i < self.sd_cards.len() - 1 {
                                ui.add_space(8.0);
                            }
                        });
                });
            }
            Screen::NetworkDrive => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select a Network Drive:");
                    ui.add_space(padding);

                    if ui.add(Button::new("Back")).clicked() {
                        self.screen = Screen::Locations;
                    }

                    ui.add_space(padding);

                    self.network_drives.iter().enumerate().for_each(
                        |(i, drive): (usize, &&str)| {
                            let button: Button<'_> =
                                Button::new(*drive).min_size(vec2(ui.available_width(), 40.0));
                            if ui.add(button).clicked() {
                                println!("{} clicked", drive);
                            }
                            if i < self.network_drives.len() - 1 {
                                ui.add_space(8.0);
                            }
                        },
                    );
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
