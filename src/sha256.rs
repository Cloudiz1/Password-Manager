// add sha256 login
// just writing this here so i dont forget later

fn hash(input: String) {
    let mut padded_input: Vec<u8> = input.into_bytes().to_vec();
    let padded_input_len: u64 = (padded_input.len() * 8) as u64;

    padded_input.push(0x80);

    while (padded_input.len() * 8) % 512 != 448 {
        padded_input.push(0);
    }

    padded_input.extend_from_slice(&padded_input_len.to_be_bytes());

    const EMPTY_STRING: String = String::new();
    let mut blocks: Vec<[String; 16]> = Vec::new();
    let mut block: [String; 16] = [EMPTY_STRING; 16];
    let mut word: String = String::new();
    
    for (i, byte) in padded_input.into_iter().enumerate() {
        word.push_str(&format!("{:08b}", byte));

        if (i + 1) % 4 == 0 {
            block[((i / 4) % 16)] = word;   // divides by four to get word iteration rather than byte iteration
                                            // modulo 16 to get iteration past first block
            word = String::new();
        }

        if (i + 1) % 64 == 0 {              // 16 words * 4 iterations per word
            blocks.push(block);
            block = [EMPTY_STRING; 16];
        }
    }


    println!("{:?}", blocks);

    // let mut input_bin: String = String::new();
    
    // for c in input.clone().into_bytes() {
    //     input_bin += &format!("0{:b}", c);
    // }

    // let org_input_bin_len: &str = &format!("{:064b}", input_bin.len());

    // input_bin += "1";

    // while input_bin.len() % 512 != 448 {
    //     input_bin += "0";
    // }

    // input_bin += org_input_bin_len;

    // let mut words: Vec<String> = Vec::new();
    // let mut word: String = String::new();
    // for c in input_bin.chars() {
    //     word.push(c);

    //     if word.len() == 32 {
    //         words.push(word);
    //         word = String::new();
    //     }
    // }
}

pub fn verify_password(input: String) -> bool {
    hash(input);
    // let mut tmp: usize = 0;
    // for word in &words {
    //     tmp += word.len();
    // }

    // println!("{}", tmp);

    // println!("{:?}", words);
    // println!("{}", input_bin_len);
    true
}

pub fn test() {
    // let input: String = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_owned();
    let input: String = "abc".to_owned();
    verify_password(input);
}