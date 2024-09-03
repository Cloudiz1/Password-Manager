use eframe::egui;
use std::mem::swap;
use crate::login;

pub fn run() -> eframe::Result {
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
    logins: login::Logins,
    new_login: login::Login
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            logins: login::get_logins(),
            new_login: login::Login {
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
            
				ui.label("Enter new login information: ");
             	ui.add(egui::TextEdit::singleline(&mut self.new_login.username).hint_text("Username: "));
              	ui.add(egui::TextEdit::singleline(&mut self.new_login.password).hint_text("Password: "));
               
				if ui.add(egui::Button::new("Add New Login")).clicked() {
					let new_login = login::Login {
						username: self.new_login.username.clone(),
						password: self.new_login.password.clone()
					};
					
					self.logins.all_logins.push(new_login);
					let mut buffer = login::Logins {
						all_logins: Vec::new()
					};
					
					swap(&mut buffer, &mut self.logins);
					
					login::write_logins(buffer);
					// println!("{:?}", self.logins);
					// login::add_new_login(self.new_login.username.clone(), self.new_login.password.clone());
				}
            });
        });
    }
}