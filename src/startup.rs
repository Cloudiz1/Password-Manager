use rand::Rng;

pub fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let mut key: String = String::new();
    for i in 0..16 {
        let number: u8 = rng.gen();
        key += &format!("{:02x}", number).to_string();
    }

    key
}