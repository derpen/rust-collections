use std::io;
mod countdown;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.pop(); // Remove \n at the end (because rust does that for some reason)

    return input;
}

fn main() {
    println!("How long would in seconds?");
    let sec = read_input();
    countdown::rs_timer(&sec);
}
