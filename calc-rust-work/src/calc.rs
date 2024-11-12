pub fn calculate(num_one: &str, num_two: &str, operator: &str) -> i64 {
    let num_one_int: i64 = num_one.parse().expect("Not a valid number");
    let num_two_int: i64 = num_two.parse().expect("Not a valid number");

    let answer;

    match operator {
        "+" => answer = num_one_int + num_two_int,
        "-" => answer = num_one_int - num_two_int,
        "*" => answer = num_one_int * num_two_int,
        "/" => answer = num_one_int / num_two_int,
        _ => return -1,
    }

    return answer;
}
