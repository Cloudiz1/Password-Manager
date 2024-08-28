use eframe::egui;

pub fn test() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Password Manager",
        options,
        Box::new(|cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    requested_login: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            requested_login: "".to_owned()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.requested_login.clone());
        });
    }
}