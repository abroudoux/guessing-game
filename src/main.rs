use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    let maximum_guesses = choose_difficulty();

    // println!("The secret number is: {secret_number}");

    loop {
        if guesses >= maximum_guesses {
            println!("You've reached the maximum of guesses!");
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
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yo win!");
                break;
            },
        }
    }

    println!("Total guesses: {guesses}");

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