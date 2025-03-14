pub fn print_fibo(n : i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        print_fibo(n - 1) + print_fibo(n - 2)
    }
}
