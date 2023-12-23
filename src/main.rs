use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::{Instant, Duration};

fn main() {
    println!("\x1b[1;34mGuess the number\x1b[0m");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    let maximum_guesses = choose_difficulty();
    let timer_duration = get_timer_duration(maximum_guesses);
    let mut timer_started = false;
    let mut timer = Instant::now();
    let mut points = 50;

    println!("\x1b[0;34mThe secret number is: {secret_number}\x1b[0m");

    loop {
        if guesses >= maximum_guesses {
            println!("\x1b[1;31mYou've reached the maximum of guesses!\x1b[0m");
            break;
        }

        if !timer_started {
            timer_started = true;
            timer = Instant::now();
        }

        if timer.elapsed() > timer_duration {
            println!("\x1b[1;31mTime's up! You took too long.\x1b[0m");
            break;
        }

        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guesses += 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("\x1b[1;31mToo small!\x1b[0m");
                points -= 2;
            }
            Ordering::Greater => {
                println!("\x1b[1;31mToo big!\x1b[0m");
                points -= 2;
            }
            Ordering::Equal => {
                show_results(timer.elapsed().as_secs(), guesses, points);
                break;
            },
        }
    }

}

fn choose_difficulty() -> u32 {

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

fn get_timer_duration(maximun_guesses: u32) -> Duration {

    match maximun_guesses {
        10 => Duration::from_secs(30),
        7 => Duration::from_secs(20),
        5 => Duration::from_secs(15),
        _ => Duration::from_secs(20),
    }
}

fn show_results(elapsed_time: u64, guesses: u32, points: i32) {

    let points_finals = points - elapsed_time as i32;

    if points_finals > 0 {
        println!("\x1b[1;32mYou win!\x1b[0m");
        println!("\x1b[0;32mTotal guesses: {guesses}\x1b[0m");
        println!("\x1b[0;32mElapsed time: {elapsed_time} seconds\x1b[0m");
        println!("\x1b[0;32mPoints: {points_finals}\x1b[0m");
    } else {
        println!("\x1b[1;31mYou lost!\x1b[0m");
    }
}
