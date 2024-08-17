mod states;
mod rounds;
mod lookup;

fn main() {
    // inputs::test();
    let test_input: &str = "abcdefghijklmnopqrstuvwxyz";
    let states: Vec<[[u8; 4]; 4]> = states::create_states(test_input);

    println!("{:?}", states[0]);
    let sub_bytes = rounds::sub_bytes(states[0], "normal");
    println!("{:?}", sub_bytes);
    let inv_sub_bytes = rounds::sub_bytes(sub_bytes, "inverse");
    println!("{:?}", inv_sub_bytes);
    // for state in &states
    // {
    //     rounds::sub_bytes(*state);
    // }
    // println!("{:?}", states);
}
