use std::fs;

fn shr(input: u32, shift: usize) -> u32 {
    let mut buffer: String = String::new();
    let input_string: String = format!("{:032b}", input).to_string();

    for i in 0..shift {
        buffer += "0";
    }

    for bit in input_string[0..input_string.len()-shift].chars() {
        buffer += &bit.to_string();
    }

    u32::from_str_radix(&buffer, 2).unwrap()
}

fn rotr(input: u32, shift: usize) -> u32 {
    let mut buffer: String = String::new();
    let input_string: String = format!("{:032b}", input).to_string();
    
    for bit in input_string[input_string.len()-shift..].chars() {
        buffer += &bit.to_string();
    } 

    for bit in input_string[0..input_string.len()-shift].chars() {
        buffer += &bit.to_string();
    }

    u32::from_str_radix(&buffer, 2).unwrap()
}

fn find_next_word(inputs: [u32; 4]) -> u32 {
    let s1 = rotr(inputs[0], 17) ^ rotr(inputs[0], 19) ^ shr(inputs[0], 10);
    let s0 = rotr(inputs[2], 7) ^ rotr(inputs[2], 18) ^ shr(inputs[2], 3);

    s1.wrapping_add(inputs[1]).wrapping_add(s0).wrapping_add(inputs[3])
}

pub fn hash(input: String) -> String {
    let mut padded_input: Vec<u8> = input.into_bytes().to_vec();
    let padded_input_len: u64 = (padded_input.len() * 8) as u64;

    padded_input.push(0x80);

    while (padded_input.len() * 8) % 512 != 448 {
        padded_input.push(0);
    }

    padded_input.extend_from_slice(&padded_input_len.to_be_bytes());

    let mut blocks: Vec<Vec<u32>> = Vec::new();
    let mut block: Vec<u32> = Vec::new();
    let mut word: String = String::new();
    
    for (i, byte) in padded_input.into_iter().enumerate() {
        word.push_str(&format!("{:08b}", byte));

        if (i + 1) % 4 == 0 {
            let static_word: &str = &word[..];
            block.push(u32::from_str_radix(static_word, 2).unwrap());   
            word = String::new();
        }

        if (i + 1) % 64 == 0 {
            blocks.push(block);
            block = Vec::new();
        }
    }

    for mut block in &mut blocks {
        while block.len() < 64 {
            let i = block.len();
            block.push(find_next_word([block[i-2], block[i-7], block[i-15], block[i-16]]));
        }
    }

    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
 
    let mut prev_hash: [u32; 8] = [0; 8];
    let mut curr_hash: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

    for block in blocks {
        let mut a = curr_hash[0];
        let mut b = curr_hash[1];
        let mut c = curr_hash[2];
        let mut d = curr_hash[3];
        let mut e = curr_hash[4];
        let mut f = curr_hash[5];
        let mut g = curr_hash[6];
        let mut h = curr_hash[7];

        for i in 0..64 {
            let sigma1: u32 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25);
            let ch: u32 = (e & f) ^ (!e & g); 
            let t1 = h.wrapping_add(sigma1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(block[i]);

            let sigma0: u32 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22); 
            let maj: u32 = (a & b) ^ (a & c) ^ (b & c);
            let t2 = sigma0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1); 
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2); 

        }

        for (i, hash) in curr_hash.iter().enumerate() {
            prev_hash[i] = *hash;
        }

        curr_hash[0] = prev_hash[0].wrapping_add(a);
        curr_hash[1] = prev_hash[1].wrapping_add(b);
        curr_hash[2] = prev_hash[2].wrapping_add(c);
        curr_hash[3] = prev_hash[3].wrapping_add(d);
        curr_hash[4] = prev_hash[4].wrapping_add(e);
        curr_hash[5] = prev_hash[5].wrapping_add(f);
        curr_hash[6] = prev_hash[6].wrapping_add(g);
        curr_hash[7] = prev_hash[7].wrapping_add(h);
    }   

    let mut output: String = String::new();

    for hash in curr_hash {
        output += &format!("{:08x}", hash).to_string();
    }

    output
}

pub fn verify_password(input: String) -> bool {
    let inputted_password = hash(input);
    let hashed_password: String = match fs::read_to_string("database/password.txt") {
        Ok(v) => v,
        Err(e) => panic!("could not read hashed password")
    };

    if inputted_password == hashed_password {
        return true;
    }

    println!("Wrong password");

    false
}

// pub fn test() {
//     // let input: String = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_owned();
//     let input: String = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_owned();
//     verify_password(input);
// }