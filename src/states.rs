pub fn create_state (input: [u8; 16]) -> [[u8; 4]; 4]
{
    let mut output: [[u8; 4]; 4] = [[b'\0'; 4]; 4]; 

    let mut i: usize = 0;
    for column in 0..4 {
        for row in 0..4 {
            output[row][column] = input[i];
            i += 1;
        }
    }

    output
}

pub fn create_states (input: &str) -> Vec<[[u8; 4]; 4]>
{
    let mut states: Vec<[[u8; 4]; 4]> = vec![];
    let mut char_buffer: [u8; 16] = [b'\0'; 16];

    let mut i: usize = 0;
    for byte in input.chars()
    {
        char_buffer[i] = byte as u8;

        i += 1;

        if i == 16
        {
            states.push(create_state(char_buffer));
            i = 0;
        }
    }

    if i != 0
    {
        let missing_len: usize = 16 - (input.len() % 16);
        
        for n in (16 - missing_len)..16
        {
            char_buffer[n] = missing_len as u8;
        }

        states.push(create_state(char_buffer));
    }

    states
}

pub fn states_to_hex(input: Vec<[[u8; 4]; 4]>) -> String
{
    let mut output: String = "".to_owned();
    for state in input
    {
        for column in 0..4
        {
            for row in 0..4
            {
                let byte_as_hex: String = format!("{:02X}", state[row][column]);
                output += &byte_as_hex;
            }
        }
    }

    output
}