extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let mut secret_number = String::new();
    let mut guess_count: u32 = 0;
    
    println!("Welcome to the reverse guessing game!");
    println!("Please provide a secret number between 1 - 100:");

    loop {
        io::stdin().read_line(&mut secret_number).expect("Error!");

        let secret_number_validation: u32 = match secret_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // :(
    }

    println!("The secret number has been set to: {}", secret_number.trim());

    loop {
        let guess: u32 = rand::thread_rng().gen_range(1, 101);

        let secret_number: u32 = match secret_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match secret_number.cmp(&guess) {
            Ordering::Less => { println!("GUESS #{}: The computer guessed {}, which was too low!", guess_count, guess); guess_count += 1; },
            Ordering::Greater => { println!("GUESS #{}: The computer guessed {}, which was too high!", guess_count, guess); guess_count += 1; },
            Ordering::Equal => {
                println!("The computer guessed {} which is CORRECT!", guess);
                println!("The computer got the correct number after {} GUESSES!", guess_count);
                break;
            },
        }
        
        
    }
}
