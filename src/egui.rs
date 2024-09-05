use eframe::egui;
use std::mem::swap;
use std::format;
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
            	application: "".to_string(),
            	username: "".to_string(),
             	password: "".to_string(),
              	id: 0
            }
        }
    }
}

// TODO: rewrite this to work with grid

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
            
				ui.label("Enter new login information: ");
				ui.add(egui::TextEdit::singleline(&mut self.new_login.application).hint_text("Application Name: "));
             	ui.add(egui::TextEdit::singleline(&mut self.new_login.username).hint_text("Username: "));
              	ui.add(egui::TextEdit::singleline(&mut self.new_login.password).hint_text("Password: "));
               
				if ui.add(egui::Button::new("Add New Login")).clicked() {
					let new_login = login::Login {
						application: self.new_login.application.clone(),
						username: self.new_login.username.clone(),
						password: self.new_login.password.clone(),
						id: self.logins.all_logins[self.logins.all_logins.len() - 1].id + 1
					};
					
					if new_login.application == "".to_string() || new_login.username == "".to_string() || new_login.password == "".to_string() {
						println!("Missing input.");
					}
					
					else {
						// println!("{:?}", new_login);
						
						self.logins.all_logins.push(new_login);
						let mut buffer = login::Logins {
							all_logins: Vec::new()
						};
						
						swap(&mut buffer, &mut self.logins);
						
						login::write_logins(buffer);
					}
					// println!("{:?}", self.logins);
					// login::add_new_login(self.new_login.username.clone(), self.new_login.password.clone());
				}

                // paint a rect behind this
                
                ui.label(""); // empty space
                
                let logins_container = egui::ScrollArea::vertical().show(ui, |ui| {
                	for credential in self.logins.all_logins.clone() {
                 		ui.label(format!("application: {}", credential.application));
                 		ui.label(format!("username: {}", credential.username));
                   		ui.label(format!("password: {}", credential.password));
                     	ui.label(format!("id: {}", credential.id));
                      	if ui.add(egui::Button::new("delete credentials")).clicked() {
                       		self.logins.all_logins.swap_remove(credential.id);
                         
                         	let mut buffer = login::Logins {
                          		all_logins: Vec::new()
                          	};
						
                          	swap(&mut buffer, &mut self.logins);
						
                           	login::write_logins(buffer);
                       	}
                 	}
                });
                
                
                

                // let logins_frame = egui::Frame::none()
                // .fill(egui::Color32::BLACK)
                // .paint(logins_container);

                // ui.painter().add(logins_frame);
            });
        });
    }
}