use std::time::{Duration, Instant};
use std::thread::sleep;

pub fn rs_timer(second: &str){
    let now = Instant::now();
    let second_int: u64 = second.parse().expect("Not valid number!");
    let countdown_duration = Duration::from_secs(second_int);

    loop {
        let elapsed = now.elapsed();
        if elapsed >= countdown_duration {
            println!("Done!");
            break;
        } else {
            let remaining = countdown_duration - elapsed;
            println!("Timer: {}", remaining.as_secs() + 1);
        }

        sleep(Duration::from_millis(1000)); // Little sleep to not spam the output
    }
}
