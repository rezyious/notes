use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut guess_counter: u32 = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // NOTICE: getting user input :
    println!("Guess the number!");

    loop {
        guess_counter += 1;

        // PERF: This way we flush the buffer for input
        // to apper on the same line as the ouput we get with `read_line`
        print!("Please input your guess: ");
        io::stdout().flush().unwrap(); // flush the output buffer!

        let mut guess = String::new();

        // NOTE: The book version which just panics on error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // NOTE: with an if let
        // if let Err(e) = io::stdin().read_line(&mut guess) {
        //     eprintln!("Unable to read the input {e}");
        // }

        // NOTE: with a match. we are not interested in how many bytes are read
        // so we handle the `OK()` like below
        // match io::stdin().read_line(&mut guess) {
        //     Ok(_) => (),
        //     Err(e) => eprintln!("Unable to read the input {e}"),
        // }

        // NOTE: with annotating the type
        // let guess: i32 = guess.trim().parse().unwrap();

        // NOTE: with turbofish operator
        // let guess = guess.trim().parse::<i32>().unwrap();
        let guess = match guess.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Not an integer");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("Correct number was {secret_number} and you guess it in {guess_counter} guesses");
}

/* {{{   IMPORTANT: BOOK FINAL VERSIN

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
}}} */
