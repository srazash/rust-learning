// in the [Cargo.toml] file there is a section called 'dependencies'
// there we have added a line: rand = "0.6.5", this tells rust that we want to use
// the external cargo library 'rand' (called a crate) as part of our project
// to use it in the project we need to call it using the following command
extern crate rand;

// import io from the standard library (std)
use std::io;
// import Rng from the external rand crate
use rand::Rng;

// new function (fn) called main
// in rust the main function is always the entry point of an application
// (this differs in a library)
fn main() {
    // print a line
    println!("Guess the number!");
    println!("Please enter your guess:");

    // declare a variable and call in rand to generate a value between 1 and 100
    // in code we do this with rand::thread_rng().gen_range(1, 101);
    // 1 indicates the starting value
    // 101 indicates the value to stop at (this value is exclusive, not inclusive)
    // which would give us 1, 2, 3 ... 98, 99, 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // declare a variable (let)
    // by default, in rust all variables are IMMUTABLE meaning thay cannot be changed
    // we can set a variable to be MUTABLE using (mut) allowing us to change it later
    // we have assigned (=) this variable to be an empty string (String::new())
    let mut guess = String::new();

    // using (io) from the standard library (std::io) we will read in the next line
    // we read in the line to the variable 'guess' using (&mut)
    // (&) refers to the fact that it is a REFERENCE, (&mut) means it is a mutable reference
    // like variables, references are immutable by default so we need to specify that the reference
    // (like the variable) is mutable. if we used (&guess) we would cause an error
    io::stdin()
        // we can break up long lines
        // this will still be treat as a single statement by rust as that is delineated by the
        // end of line symbol (;)
        .read_line(&mut guess)
        .expect("Failed to read line.");

    // print a line
    // the placeholder ({}) inside the printed string allows us to insert a variable
    // here we provide the (guess) variable to be printed as part of the string
    println!("You guessed: {} and the secret number is: {}.", guess, secret_number);
}
