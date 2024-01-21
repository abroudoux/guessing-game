use std::io;
use std::time::Duration;

pub fn choose_difficulty() -> u32 {
    println!("Choose the difficulty level:");
    println!("1. Easy (10 guesses)");
    println!("2. Medium (7 guesses)");
    println!("3. Hard (5 guesses)");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => return 10,
            Ok(2) => return 7,
            Ok(3) => return 5,
            _ => {
                println!("\x1b[0;31mInvalid choice. Please try again\x1b[0m");
                continue;
            }
        }
    }
}

pub fn get_timer_duration(maximun_guesses: u32) -> Duration {

    match maximun_guesses {
        10 => Duration::from_secs(30),
        7 => Duration::from_secs(20),
        5 => Duration::from_secs(15),
        _ => Duration::from_secs(20),
    }
}

pub fn show_results(elapsed_time: u64, guesses: u32, points: i32) {

    let points_finals = (points - elapsed_time as i32) * 10;

    if points_finals > 0 {
        println!("\x1b[1;32mYou win!\x1b[0m");
        println!("\x1b[0;32mTotal guesses: {guesses}\x1b[0m");
        println!("\x1b[0;32mElapsed time: {elapsed_time} seconds\x1b[0m");
        println!("\x1b[0;32mPoints: {points_finals}\x1b[0m");
    } else {
        println!("\x1b[1;31mYou lost!\x1b[0m");
    }
}