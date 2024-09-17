use std::io::stdin;
use std::fs;

mod states;
mod cipher;
mod lookup;
mod egui;
mod login;
mod sha256;
mod startup;

fn main() {
    let database_path: String = "database".to_owned();

    if !fs::metadata(database_path.clone()).is_ok() {
        match fs::create_dir(database_path.clone()) {
            Ok(v) => v,
            Err(e) => panic!("Error creating database directory ({}): {} ", database_path, e)
        }

        println!("Generating key");
        let key = startup::generate_key();
        fs::write(database_path.clone() + "/key.txt", key.as_bytes());

        println!("Generating initialization vector");
        let init_vec = startup::generate_key(); 
        fs::write(database_path.clone() + "/IV.txt", init_vec.as_bytes());

        println!("Encrypting empty logins file");
        let content: String = fs::read_to_string("src/assets/empty_logins.txt").unwrap();
        let content_slice: &str = &content[..];
        let encrypted_text: String = cipher::encrypt_str(content_slice);
        fs::write(database_path.clone() + "/logins.txt", encrypted_text.as_bytes());

        let mut inputted_password = "".to_owned();

        while inputted_password == "" {
            println!("Enter login password: ");
            stdin().read_line(&mut inputted_password).unwrap(); // comes with two extra characters for some reason
            inputted_password = inputted_password.trim_end().to_string();

            if inputted_password == "" {
                println!("No password inputted. ");
            }
        }

        inputted_password = (&inputted_password[0..]).to_string(); // removes the random two characters that come with read_line()

        println!("Hashing password");
        let hashed_password = sha256::hash(inputted_password);
        println!("{}", hashed_password);
        fs::write(database_path.clone() + "/password.txt", hashed_password.as_bytes());


    }

    egui::run();
}
