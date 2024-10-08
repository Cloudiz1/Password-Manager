use eframe::egui;
use std::{format, fs, thread};
use std::mem::swap;
use std::time::Duration;
use crate::login;
use crate::sha256;

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Password Manager",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
	scene: usize,
    logins: login::Logins,
    new_login: login::Login,
    show_login: bool,
    sign_in_password: String,
    new_password: String,
    confirm_password: String,
    show_message: bool,
    currently_showing_message: bool,
    message: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
			scene: 0,
            logins: login::get_logins(),
            new_login: login::Login {
            	application: "".to_string(),
            	username: "".to_string(),
             	password: "".to_string(),
              	id: 0
            },
            show_login: false,
            sign_in_password: "".to_string(),
            new_password: "".to_string(),
            confirm_password: "".to_string(),
            show_message: false,
            currently_showing_message: false,
            message: "".to_string()
        }
    }
}

impl MyApp {
	fn update_ids(&mut self) {
		for i in 0..self.logins.all_logins.len() {
			self.logins.all_logins[i].id = i as usize;
		}
	}
}

impl MyApp {
    fn attempt_show_message(&mut self, ui: &mut egui::Ui) {
        if self.show_message {
            // put this in a nice looking place
            // ui.heading(self.message.clone());

            let window_height = ui.available_height();
            let pos = egui::Pos2::new(10.0, window_height);
            ui.painter().text(pos, egui::Align2::LEFT_BOTTOM, self.message.clone(), egui::FontId::default(), egui::Color32::GRAY);

            if self.currently_showing_message {
                thread::sleep(Duration::from_secs(1));
                self.show_message = false;
                self.currently_showing_message = false;
            }
            else {
                self.currently_showing_message = true;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.scene {
            0 => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.attempt_show_message(ui);
                    ui.vertical(|ui| {
                        ui.add_space(215.0);
                        ui.horizontal(|ui| {
                            ui.add_space(175.0);
                            ui.heading("Login: ");
                        });

                        ui.add_space(7.5);

                        ui.horizontal(|ui| {
                            ui.add_space(175.0);
                            
                            ui.add(egui::TextEdit::singleline(&mut self.sign_in_password).hint_text("Password:"));
                            if ui.add(egui::Button::new("Sign in")).clicked() {
                                if sha256::verify_password(self.sign_in_password.clone()) {
                                    self.scene += 1;
                                }
                                else {
                                    self.show_message = true;
                                    self.message = "Wrong Password.".to_string()
                                }
                            }   
                        });
                    })
                });
            },

            1 => {
                egui::SidePanel::left("add_new_login_form")
                .exact_width(325.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        self.attempt_show_message(ui);
                        ui.label("Enter new login information: ");
                        ui.add(egui::TextEdit::singleline(&mut self.new_login.application).hint_text("Application Name: "));
                        ui.add(egui::TextEdit::singleline(&mut self.new_login.username).hint_text("Username: "));
                        ui.add(egui::TextEdit::singleline(&mut self.new_login.password).hint_text("Password: "));
                        
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("Add New Login")).clicked() {
                                let new_login = login::Login {
                                    application: self.new_login.application.clone(),
                                    username: self.new_login.username.clone(),
                                    password: self.new_login.password.clone(),
                                    id: 0 	// placeholder; cant get accurate id if .all_logins is empty
                                            // instead, just calling self.update_ids();
                                };
                                
                                if new_login.application == "".to_string() || new_login.username == "".to_string() || new_login.password == "".to_string() {
                                    println!("Missing input.");
                                }
                                
                                else {
                                    self.logins.all_logins.push(new_login);
                                    let mut buffer = login::Logins {
                                        all_logins: Vec::new()
                                    };
                                    
                                    swap(&mut buffer, &mut self.logins);
                                    
                                    login::write_logins(buffer);
                                    self.logins = login::get_logins();
                
                                    self.update_ids();
                                }
                            }
        
                            ui.add_space(65.0);
                            
                            ui.checkbox(&mut self.show_login, "Display credentials");
                        });

                        ui.add_space(20.0);

                        ui.label("Change password: ");
                        ui.add(egui::TextEdit::singleline(&mut self.new_password).hint_text("New password: "));
                        ui.add(egui::TextEdit::singleline(&mut self.confirm_password).hint_text("Confirm password: "));

                        if ui.add(egui::Button::new("Change Password")).clicked() {
                            if self.new_password == self.confirm_password {
                                let hashed_password = sha256::hash(self.new_password.clone()).clone();
                                let _ = fs::write("database/password.txt", hashed_password.as_bytes());
                                
                                self.message = "Password successfully changed.".to_string();
                                self.show_message = true;
                            }
                            else {
                                self.message = "Passwords do not match.".to_string();
                                self.show_message = true;
                            }
                        };
                    });
                });
        
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if self.show_login {
                            let logins_container = egui::ScrollArea::vertical()
                            .auto_shrink([false; 2])
                            .max_width(f32::INFINITY)
                            .max_height(ui.available_height());
        
                            logins_container.show(ui, |ui| {
                                for credential in self.logins.all_logins.clone() {
                                    ui.label(format!("application: {}", credential.application));
                                    ui.label(format!("username: {}", credential.username));
                                    ui.label(format!("password: {}", credential.password));
        
                                    if ui.add(egui::Button::new("delete credentials")).clicked() {
                                        self.logins.all_logins.swap_remove(credential.id);
        
                                        let mut buffer = login::Logins {
                                            all_logins: Vec::new()
                                        };
                                        
                                        swap(&mut buffer, &mut self.logins);
        
                                        login::write_logins(buffer);
                                        self.logins = login::get_logins();
        
                                        self.update_ids();
                                    }
                                    
                                    ui.add_space(8.0);
                                }
                            });
                        }
                    })
                });
            },

            _ => {
                panic!("scene number went out of range");
            }
        }
    }
}