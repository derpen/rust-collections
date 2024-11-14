pub fn calculate(num_one: &str, num_two: &str, operator: &str) -> i64 {
    let num_one_int: i64 = num_one.parse().expect("Not a valid number");
    let num_two_int: i64 = num_two.parse().expect("Not a valid number");

    let answer;

    answer = match operator {
        "+" => num_one_int + num_two_int,
        "-" => num_one_int - num_two_int,
        "*" => num_one_int * num_two_int,
        "/" => num_one_int / num_two_int,
        //_ => return -1,
        _ =>  {
            print!("ERROR----------Please input a valid operator (+ - * /)-----------------\n"); 
            0
        }
    };

    return answer;
}
