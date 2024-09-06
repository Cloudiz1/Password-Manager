mod states;
mod cipher;
mod lookup;
mod egui;
mod login;
mod sha256;

// // for printing 2d arrays
// fn pretty_print(input: [[u8; 4]; 4])
// {
//     for row in input
//     {
//         println!("{:?}", row);
//     }
// }

fn main() {
    egui::run();
    // login::test();
    
    
    
    // // mix columns testing
    // let test_input: [[u8; 4]; 4] = [
    //     [219, 242, 1, 198],
    //     [19, 10, 1, 198],
    //     [83, 34, 1, 198],
    //     [69, 92, 1, 198]
    // ];

    // let output = rounds::mix_columns(test_input, "normal");
    // pretty_print(output);
    // pretty_print(rounds::mix_columns(output, "inverse"));
    // // inputs::test();
    // let test_input: &str = "abcdefghijklmnopqrstuvwxyz";
    // let states: Vec<[[u8; 4]; 4]> = states::create_states(test_input);

    // let gf_out: u8 = rounds::gfm(255, 11);
    // println!("{}", gf_out);

    // // tests shift rows and inv shift rows
    // println!("{:?}", states[0]);
    // let shifted_rows = rounds::shift_rows(states[0], "normal");
    // println!("{:?}", shifted_rows);
    // println!("{:?}", rounds::shift_rows(shifted_rows, "inverse"));

    // // tests sub bytes and inverse sub bytes
    // println!("{:?}", states[0]);
    // let sub_bytes = rounds::sub_bytes(states[0], "normal");
    // println!("{:?}", sub_bytes);
    // let inv_sub_bytes = rounds::sub_bytes(sub_bytes, "inverse");
    // println!("{:?}", inv_sub_bytes);




    // for state in &states
    // {
    //     rounds::sub_bytes(*state);
    // }
    // println!("{:?}", states);
}
