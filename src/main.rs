use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::{Instant, Duration};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    let maximum_guesses = choose_difficulty();
    let timer_duration = get_timer_duration(maximum_guesses);
    let mut timer_started = false;
    let mut timer = Instant::now();
    let mut points = 50;

    println!("The secret number is: {secret_number}");

    loop {
        if guesses >= maximum_guesses {
            println!("You've reached the maximum of guesses!");
            break;
        }

        if !timer_started {
            timer_started = true;
            timer = Instant::now();
        }

        if timer.elapsed() > timer_duration {
            println!("Time's up! You took to long.");
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
                println!("Too small!");
                points -= 2;
            }
            Ordering::Greater => {
                println!("Too big!");
                points -= 2;
            }
            Ordering::Equal => {
                println!("Yo win!");
                show_results(timer.elapsed().as_secs(), guesses, points);
                break;
            },
        }
    }

}

fn choose_difficulty() -> u32 {

    loop {
        println!("Choose the difficulty level:");
        println!("1. Easy (10 guesses)");
        println!("2. Medium (7 guesses)");
        println!("3. Hard (5 guesses)");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => return 10,
            Ok(2) => return 7,
            Ok(3) => return 5,
            _ => {
                println!("Invalid choice. Please try again");
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
    println!("Total guesses: {guesses}");
    println!("Elapsed time: {} seconds", elapsed_time);
    println!("Points: {}", points_finals);
}
