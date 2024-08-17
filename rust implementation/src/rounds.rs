use crate::lookup;
extern crate hex;

pub fn sub_bytes(input: [[u8; 4]; 4], mode: &str) -> [[u8; 4]; 4]
{
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4];

    for row in 0..input.len()
    {
        for column in 0..input[0].len()
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