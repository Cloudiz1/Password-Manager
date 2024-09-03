use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use crate::cipher;

// get serde json working: Done
// complete add_new_login: DONE
// encrpyt

pub const LOGIN_PATH: &str = "logins.txt";

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
	pub username: String,
	pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logins {
	pub all_logins: Vec<Login>
}

pub fn get_logins() -> Logins
{
	let contents = fs::read_to_string(LOGIN_PATH).unwrap();
	let logins_str = cipher::decrypt_str(contents);
	match serde_json::from_str(&logins_str)
		{
			Ok(v) => v,
			Err(e) => panic!("{:?}", e)
		}
}

pub fn write_logins(logins: Logins) {
	let json_string = serde_json::to_string(&logins).unwrap();
	let encrypted_logins = cipher::encrypt_str(&json_string);
	
	fs::write(LOGIN_PATH, encrypted_logins.as_bytes());
}

pub fn test()
{
	
	
	// let logins = get_logins(LOGIN_PATH);
	// println!("{:?}", logins.all_logins[0].username);
	

	// let content: String = fs::read_to_string("tmp.txt").unwrap();
	// let content_slice: &str = &content[..];
	// let encrypted_text: String = cipher::encrypt_str(content_slice);
	// fs::write("logins.txt", encrypted_text.as_bytes());
}