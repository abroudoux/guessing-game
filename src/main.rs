use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

mod game_logic;

fn main() {
    println!("\x1b[1;34mGuess the number\x1b[0m");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    let maximum_guesses = game_logic::choose_difficulty();
    let timer_duration = game_logic::get_timer_duration(maximum_guesses);
    let mut timer_started = false;
    let mut timer = Instant::now();
    let mut points = 50;

    // println!("\x1b[0;34mThe secret number is: {secret_number}\x1b[0m");

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

        if guess < 1 || guess > 100 {
            println!("\x1b[0;31mPlease enter a number between 1 and 100.\x1b[0m");
            continue;
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
                game_logic::show_results(timer.elapsed().as_secs(), guesses, points);
                break;
            },
        }
    }

}
