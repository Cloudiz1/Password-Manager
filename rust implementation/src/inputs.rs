fn create_state (input: [char; 16]) -> [[char; 4]; 4]
{
    let mut output: [[char; 4]; 4] = [['a'; 4]; 4]; 

    let mut i: usize = 0;
    for column in 0..4 {
        for row in 0..4 {
            output[row][column] = input[i];
            i += 1;
        }
    }

    return output;
}

fn create_states (input: &str) -> Vec<[[char; 4]; 4]>
{
    let mut states: Vec<[[char; 4]; 4]> = vec![];
    let mut char_buffer: [char; 16] = ['a'; 16];

    let mut i: usize = 0;
    for byte in input.chars()
    {
        char_buffer[i] = byte;

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
            char_buffer[n] = missing_len.to_string().parse().unwrap();
        }

        states.push(create_state(char_buffer));
    }

    return states;
}

// pub fn test() {
//     // // tests creates_states()
//     // let test_input: &str = "abcdefghijklmnopqrstuvwxyz";
//     // println!("{:#?}", create_states(test_input));

//     // // tests just create_state()
//     // let test_input: [char; 16] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p']; 
//     // println!("{:?}", create_state(test_input));
// }