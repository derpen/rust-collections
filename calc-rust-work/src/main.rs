use::std::io;
mod calc;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.pop(); // Remove \n at the end (because rust does that for some reason)


    return input;
}

fn main() {
    // Graphical interface soon, probably with OGL or Raylib
    println!("Simple Integer Calc \n");

    loop {
        println!("Input first number");

        let first_n = read_input();

        println!("Input operation type"); // Todo: custom operator

        let operator = read_input();

        println!("Input second number");

        let second_n = read_input();

        let result = calc::calculate(&first_n, &second_n, &operator);

        print!("Your result: {} {} {} = {} \n", first_n, operator, second_n, result); // Result here
    }
}
