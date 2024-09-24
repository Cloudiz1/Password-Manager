use crate::lookup;
use crate::states;
use std::fs;
use std::str::from_utf8;

fn sub_bytes(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4];

    for row in 0..4
    {
        for column in 0..4
        {
            let index: usize = input[row][column].into();

            let mut sub_byte = match u8::from_str_radix(lookup::SBOX[index], 16)
            {
                Ok(v) => v,
                Err(e) => panic!("Error converting hex to u8: {:?}", e)
            };   

            if mode == "inverse"
            {
                sub_byte = match u8::from_str_radix(lookup::INVERSE_SBOX[index], 16)
                {
                    Ok(v) => v,
                    Err(e) => panic!("Error converting hex to u8: {:?}", e)
                };   
            }

            output[row][column] = sub_byte
        }
    }

    output
}

fn shift_rows(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4];

    let mut i: i8 = 0;

    for row in 0..4
    {
        for column in 0..4
        {
            let mut rotation_index: i8 = column;
            
            if mode == "normal"
            {
                rotation_index += i; // shifts left
            }
            else if mode == "inverse"
            {
                rotation_index -= i; // shifts right
            }
            
            if rotation_index < 0
            {
                rotation_index += 4;
            }
            else if rotation_index > 3
            {
                rotation_index -= 4;
            }
            
            output[row][column as usize] = input[row][rotation_index as usize];
        }
        i += 1;
    }

    output
}

fn gfm(byte: u8, num: u8) -> u8 {
    match num
    {
        2 => {
            let result = byte << 1;

            if byte & 0x80 != 0
            {
                result ^ 0x1B
            }
            else
            {
                result
            }
        }
        
        3 => {
            gfm(byte, 2) ^ byte
        }
 
        9 => {
            let mut result = gfm(byte, 2);
            result = gfm(result, 2);
            gfm(result, 2) ^ byte
        }

        11 => {
            let mut result = gfm(byte, 2);
            result = gfm(result, 2) ^ byte;
            gfm(result, 2) ^ byte
        }

        13 => {
            let mut result = gfm(byte, 2) ^ byte;
            result = gfm(result, 2);
            gfm(result, 2) ^ byte
        }

        14 => {
            let mut result = gfm(byte, 2) ^ byte;
            result = gfm(result, 2) ^ byte;
            gfm(result, 2)
        }

        _ => {
            panic!("Not implemented");
        }
    }
}

fn mix_columns(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4];

    for column in 0..4
    {
        let mut column_buffer: [u8; 4] = [b'\0'; 4];

        for row in 0..4
        {
            column_buffer[row] = input[row][column];
        }

        if mode == "normal"
        {
            output[0][column] = gfm(column_buffer[0], 2) ^ gfm(column_buffer[1], 3) ^ column_buffer[2] ^ column_buffer[3];
            output[1][column] = gfm(column_buffer[1], 2) ^ gfm(column_buffer[2], 3) ^ column_buffer[3] ^ column_buffer[0];
            output[2][column] = gfm(column_buffer[2], 2) ^ gfm(column_buffer[3], 3) ^ column_buffer[0] ^ column_buffer[1];
            output[3][column] = gfm(column_buffer[3], 2) ^ gfm(column_buffer[0], 3) ^ column_buffer[1] ^ column_buffer[2];
        }
        else if mode == "inverse"
        {
            output[0][column] = gfm(column_buffer[0], 14) ^ gfm(column_buffer[1], 11) ^ gfm(column_buffer[2], 13) ^ gfm(column_buffer[3], 9);
            output[1][column] = gfm(column_buffer[1], 14) ^ gfm(column_buffer[2], 11) ^ gfm(column_buffer[3], 13) ^ gfm(column_buffer[0], 9);
            output[2][column] = gfm(column_buffer[2], 14) ^ gfm(column_buffer[3], 11) ^ gfm(column_buffer[0], 13) ^ gfm(column_buffer[1], 9);
            output[3][column] = gfm(column_buffer[3], 14) ^ gfm(column_buffer[0], 11) ^ gfm(column_buffer[1], 13) ^ gfm(column_buffer[2], 9);
        }
    }

    output
}

fn key_to_state(key: &str) -> [[u8; 4]; 4] {
    let mut return_state: [[u8; 4]; 4] = [[b'\0'; 4]; 4];
    let mut tmp: String = String::new();

    for (i, c) in key.chars().enumerate()
    {
        tmp.push(c);

        if i % 2 != 0
        {
            return_state[((i-1)/2)/4][((i-1)/2)%4] = match u8::from_str_radix(&tmp, 16) 
            {
                Ok(v) => v,
                Err(e) => panic!("Error converting hex to u8: {:?}", e)
            };
            
            tmp = "".to_string();
        }
    }   

    return_state
}

const KEY_PATH: &str = "database/key.txt";
fn generate_keys() -> Vec<[[u8; 4]; 4]> {
    let key = match fs::read_to_string(KEY_PATH)
    {
        Ok(v) => v,
        Err(e) => panic!("failed to read file: {:?}", e)
    };

    let org_key_state = key_to_state(&key);

    let mut words: Vec<[u8; 4]> = vec![];

    for word in org_key_state
    {
        words.push(word);
    }

    for i in 4..44 // 4 words * 10 rounds; +4 on start and end cause of org. key
    {
        if i % 4 == 0
        {   
            let rot_word: [u8; 4] = [words[i-1][1], words[i-1][2], words[i-1][3], words[i-1][0]];
            let mut sub_bytes: [u8; 4] = [b'\0'; 4];

            for (j, byte) in rot_word.iter().enumerate()
            {
                let sub_byte = match u8::from_str_radix(lookup::SBOX[*byte as usize], 16)
                {
                    Ok(v) => v,
                    Err(e) => panic!("Error converting hex to u8: {:?}", e)
                };

                sub_bytes[j] = sub_byte;
            }

            sub_bytes[0] ^= lookup::RCON[(i/4)-1];

            let mut output: [u8; 4] = [b'\0'; 4];
            for (j, byte) in sub_bytes.iter().enumerate()
            {
                output[j] = byte ^ words[i-4][j];
            }

            words.push(output);
        }
        else 
        {
            let mut output: [u8; 4] = [b'\0'; 4];
            for (i, (byte1, byte2)) in words[i-1].iter().zip(words[i-4].iter()).enumerate()
            {
                output[i] = byte1 ^ byte2;
            }

            words.push(output);
        }
    }

    
    let mut output_keys: Vec<[[u8; 4]; 4]> = vec![];
    
    let mut tmp_key_buffer: [[u8; 4]; 4] = [[b'\0'; 4]; 4];
    for (i, word) in words.into_iter().enumerate()
    {
        for j in 0..4
        { 
            tmp_key_buffer[j][i%4] = word[j];
        }
        
        if (i + 1) % 4 == 0
        {
            output_keys.push(tmp_key_buffer);
        }
    }

    output_keys
}

fn xor_state(input: [[u8; 4]; 4], key: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4];
    for i in 0..4
    {
        for j in 0..4
        {
            output[i][j] = input[i][j] ^ key[i][j];
        }
    } 

    output
}

const IV_PATH: &str = "database/IV.txt";
pub fn encrypt_str(input: &str) -> String {
    let mut output: Vec<[[u8; 4]; 4]> = vec![]; 
    let mut states = states::create_states(input);
    let keys = generate_keys();

    let iv_string = match fs::read_to_string(IV_PATH) {
        Ok(v) => v,
        Err(e) => panic!("failed to read file: {:?}", e)
    };

    let iv = key_to_state(&iv_string);

    states[0] = xor_state(states[0], iv);

    for (state_number, mut state) in states.into_iter().enumerate() {
        if state_number != 0 {
            state = xor_state(state, output[state_number - 1]);
        }

        let mut cipher_text = xor_state(state, keys[0]);

        for i in 1..10
        {
            cipher_text = sub_bytes(cipher_text, "normal");
            cipher_text = shift_rows(cipher_text, "normal");
            cipher_text = mix_columns(cipher_text, "normal");
            cipher_text = xor_state(cipher_text, keys[i]);
        }

        cipher_text = sub_bytes(cipher_text, "normal");
        cipher_text = shift_rows(cipher_text, "normal");
        cipher_text = xor_state(cipher_text, keys[keys.len() - 1]);

        output.push(cipher_text);
    }

    states::states_to_hex(output)
}

pub fn decrypt_str(input: String) -> String {
    let mut output: Vec<[[u8; 4]; 4]> = vec![];

    let mut states: Vec<[[u8; 4]; 4]> = vec![];
    let mut tmp_array: [u8; 16] = [b'\0'; 16];
    let mut buffer: String = String::new();
    let mut count: usize = 0;

    for (i, c) in input.chars().enumerate()
    {
        buffer.push(c);

        if i % 2 != 0
        {
            let static_buffer = &*buffer;
            tmp_array[count] = match u8::from_str_radix(static_buffer, 16)
            {
                Ok(v) => v,
                Err(e) => panic!("Error converting hex to u8: {:?}", e)
            };

            buffer = "".to_string();

            count += 1;
        }

        if count == 16
        {
            states.push(states::create_state(tmp_array));
            count = 0;
        }
    }

    let keys = generate_keys();
    
    let mut previous_state: [[u8; 4]; 4] = [[b'\0'; 4]; 4];
    for (state_number, state) in states.into_iter().enumerate()
    {
        let mut deciphered_state: [[u8; 4]; 4] = [[b'\0'; 4]; 4];

        for (i, key) in keys.iter().rev().enumerate()
        {
            if i == 0
            {
                deciphered_state = xor_state(state, *key);
                deciphered_state = shift_rows(deciphered_state, "inverse");
                deciphered_state = sub_bytes(deciphered_state, "inverse");
            }
            else if i > 0 && i < 10
            {
                deciphered_state = xor_state(deciphered_state, *key);
                deciphered_state = mix_columns(deciphered_state, "inverse");
                deciphered_state = shift_rows(deciphered_state, "inverse");
                deciphered_state = sub_bytes(deciphered_state, "inverse");
            }
            else
            {
                deciphered_state = xor_state(deciphered_state, *key);
            }
        }

        if state_number != 0 {
            deciphered_state = xor_state(deciphered_state, previous_state);
        }

        previous_state = state;
        
        output.push(deciphered_state);
    }
    
    let mut byte_array: Vec<u8> = vec![];

    let iv_string = match fs::read_to_string(IV_PATH) {
        Ok(v) => v,
        Err(e) => panic!("failed to read file: {:?}", e)
    };

    let iv = key_to_state(&iv_string);
    output[0] = xor_state(output[0], iv);

    for state in output
    {
        for column in 0..4
        {
            for row in 0..4
            {
                byte_array.push(state[row][column]);
            }
        }
    }

    // removes the extra padding
    if byte_array[byte_array.len() - 1] <= 16
    {
        for _i in 1..byte_array[byte_array.len() - 1] + 1
        {
            byte_array.pop();
        }
    }

    from_utf8(&byte_array).unwrap().to_string()
}