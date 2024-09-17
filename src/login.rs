use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;
use crate::cipher;

pub const LOGIN_PATH: &str = "database/logins.txt";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
	pub application: String,
	pub username: String,
	pub password: String,
	pub id: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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