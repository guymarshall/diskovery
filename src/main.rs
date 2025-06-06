use std::{
    fs::{DirEntry, ReadDir, read_dir},
    process::exit,
};

use eframe::{CreationContext, NativeOptions, egui};
use egui::{Button, Ui, vec2};

fn list_drives() -> Vec<String> {
    read_dir("/sys/block")
        .into_iter()
        .flat_map(|entries: ReadDir| entries.flatten())
        .filter_map(|entry: DirEntry| entry.file_name().into_string().ok())
        .filter(|file_name: &String| {
            !file_name.starts_with("loop") && !file_name.starts_with("ram")
        })
        .map(|file_name: String| format!("/dev/{}", file_name))
        .collect()
}

enum Screen {
    DriveList,
    DriveDetail { drive: String },
}

struct MyApp {
    screen: Screen,
    drives: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let drives: Vec<String> = list_drives();

        Self {
            screen: Screen::DriveList,
            drives,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let padding: f32 = 16.0;

        let mut next_screen: Option<Screen> = None;

        match &self.screen {
            Screen::DriveList => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading("Select a drive:");
                    ui.add_space(padding);

                    if ui.add(Button::new("Exit")).clicked() {
                        exit(0);
                    }

                    ui.add_space(padding);

                    let button_height: f32 = 40.0;
                    let button_spacing: f32 = 8.0;

                    for (i, drive) in self.drives.iter().enumerate() {
                        let button: Button<'_> =
                            Button::new(drive).min_size(vec2(ui.available_width(), button_height));
                        if ui.add(button).clicked() {
                            next_screen = Some(Screen::DriveDetail {
                                drive: drive.clone(),
                            });
                        }
                        if i < self.drives.len() - 1 {
                            ui.add_space(button_spacing);
                        }
                    }

                    ui.add_space(padding);
                });
            }
            Screen::DriveDetail { drive } => {
                egui::CentralPanel::default().show(context, |ui: &mut Ui| {
                    ui.heading(format!("Drive: {}", drive));
                    ui.add_space(padding);

                    if ui.add(Button::new("Back")).clicked() {
                        next_screen = Some(Screen::DriveList);
                    }

                    ui.add_space(padding);

                    if ui.add(Button::new("Scan")).clicked() {
                        println!("Scan clicked for {}", drive);
                    }
                });
            }
        }

        if let Some(screen) = next_screen {
            self.screen = screen;
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
