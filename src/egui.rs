use eframe::egui;
use crate::login;

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
    requested_login: String,
    new_login: Login
}

struct Login {
	username: String,
	password: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            requested_login: "".to_owned(),
            new_login: Login {
            	username: "".to_string(),
             	password: "".to_string()
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
            	ui.label("Select a login: ");
            
            	egui::ComboBox::from_label("")
                .selected_text(&self.requested_login)
            	.show_ui(ui, |ui| {
             		// ui.selectable_value(&mut self.requested_login, "test".to_owned(), "test");
             	});

	            ui.label("");
             
				ui.label("Enter new login information: ");
             	let inputted_username = ui.add(egui::TextEdit::singleline(&mut self.new_login.username).hint_text("Username: "));
              	let intputted_password = ui.add(egui::TextEdit::singleline(&mut self.new_login.password).hint_text("Password: "));
               
				if ui.add(egui::Button::new("Add New Login")).clicked() {
					login::add_new_login(self.new_login.username.clone(), self.new_login.password.clone());
				}
            });
        });
    }
}