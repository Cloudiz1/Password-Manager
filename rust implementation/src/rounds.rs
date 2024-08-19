use crate::lookup;
extern crate hex;

pub fn sub_bytes(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4]
{
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4];

    for row in 0..4
    {
        for column in 0..4
        {
            let index: usize = input[row][column].into();

            let mut sub_byte = hex::decode(lookup::SBOX[index]);

            if mode == "inverse"
            {
                sub_byte = hex::decode(lookup::INVERSE_SBOX[index]);
            }
            
            match sub_byte
            {
                Ok(v) => output[row][column] = v[0],
                Err(e) => println!("Error: {}", e)
            };
        }
    }

    output
}

pub fn shift_rows(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4]
{
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

fn gfm(byte: u8, num: u8) -> u8
{
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

pub fn mix_columns(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4]
{
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