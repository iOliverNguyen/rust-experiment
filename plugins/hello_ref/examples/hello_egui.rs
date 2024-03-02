use eframe::egui;
use misc::planets::ListPlanets;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800., 600.]),
        ..Default::default()
    };
    eframe::run_native(
        "Solar System",
        options,
        Box::new(|cc| Box::new(PlanetsApp::new())),
    )
}

pub struct PlanetsApp {
    planets: Box<ListPlanets>,
}

impl PlanetsApp {
    pub fn new() -> Self {
        Self {
            planets: Box::new(ListPlanets::init()),
        }
    }
}

impl eframe::App for PlanetsApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .height_range(40.0..=40.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label("Solar System App");
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .height_range(32.0..=32.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Status: ");
                    ui.label("OK");
                });
            });

        egui::SidePanel::left("left_panel")
            .width_range(200.0..=200.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label("◀");
                    ui.label("left")
                });
            });

        egui::SidePanel::right("right_panel")
            .width_range(200.0..=200.0)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("▶");
                    ui.label("right");
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("EARTH");
            })
        });
    }
}
