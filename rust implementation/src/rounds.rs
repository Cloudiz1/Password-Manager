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

    return output;
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

    return output;
}