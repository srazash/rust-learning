extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut secret_number = String::new();
    let mut guess_count: u32 = 0;
    let mut rand_guess: Vec<u32> = Vec::new();

    println!("Welcome to the reverse guessing game!");
    println!("Please provide a secret number between 1 - 100:");
    
    // loop to get user input, will loop until the provided value is between 1 - 100
    loop {
        secret_number = "0".to_string();

        io::stdin().read_line(&mut secret_number).expect("Error!");

        let secret_number_validation: u32 = match secret_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if secret_number_validation >= 1 && secret_number_validation <= 100 {
            secret_number = secret_number.to_string();
            break;
        } else {
            println!("Please provide a secret number between 1 - 100:");
        }

    }

    println!("The secret number has been set to: {}", secret_number.trim());

    // loop to guess what the secret number is, will loop until the randomly guessed value matches the user inputted secret number
    // THERE IS A MAJOR ISSUE WITH THIS AS IT CURRENTLY CAN GUESS INCORRECT VALUES MULTIPLE TIMES!
    loop {
        let guess: u32 = rand::thread_rng().gen_range(1, 101);

        // add the random guess to the vector
        // we want to compare the current guess against previous guesses
        // if the guess hasn't occured before allow the loop to continue
        // otherwise we need to regen the guess
        rand_guess.push(guess);

        let secret_number: u32 = match secret_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match secret_number.cmp(&guess) {
            Ordering::Less => { println!("GUESS #{}: The computer guessed {}, which was too high!", guess_count, guess); guess_count += 1; },
            Ordering::Greater => { println!("GUESS #{}: The computer guessed {}, which was too low!", guess_count, guess); guess_count += 1; },
            Ordering::Equal => {
                println!("The computer guessed {} which is CORRECT!", guess);
                println!("The computer got the correct number after {} GUESSES!", guess_count);
                break;
            },
        }
        
        
    }
}
