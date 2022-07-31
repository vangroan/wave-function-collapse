use eframe::egui;

mod loader;

pub use loader::load_tileset_file;

pub struct WaveApp {
    name: String,
    age: i32,
}

impl WaveApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        WaveApp {
            name: "".to_string(),
            age: 0,
        }
    }

    pub fn run() {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Wave Function Collapse",
            native_options,
            Box::new(|cc| Box::new(Self::new(cc))),
        );
    }
}

impl eframe::App for WaveApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
