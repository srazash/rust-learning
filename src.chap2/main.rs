/*
    in the [Cargo.toml] file there is a section called 'dependencies'
    there we have added a line: rand = "0.6.5", this tells rust that we want to use
    the external cargo library 'rand' (called a crate) as part of our project
    to use it in the project we need to call it using the following command
*/
extern crate rand;

// import io and cmp::Ordering from the standard library (std)
use std::io;
use std::cmp::Ordering;

// import Rng from the external rand crate
use rand::Rng;

/*
    new function (fn) called main
    in rust the main function is always the entry point of an application
    (this differs in a library)
*/
fn main() {
    // print a line
    println!("Guess the number!");
    println!("Any value between 1 - 100");

    /*
        declare a variable and call in rand to generate a value between 1 and 100
        in code we do this with rand::thread_rng().gen_range(1, 101);
        1 indicates the starting value
        101 indicates the value to stop at (this value is exclusive, not inclusive)
        which would give us 1, 2, 3 ... 98, 99, 100
    */
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // print a line
    // commenting out this line
    // println!("DEBUG: The secret number is: {}", secret_number);

    /*
        everything within the loop code block will run forever
        ...or until the program is manually stopped by the user
    */
    loop {
        /*
            declare a variable (let)
            by default, in rust all variables are IMMUTABLE meaning thay cannot be changed
            we can set a variable to be MUTABLE using (mut) allowing us to change it later
            we have assigned (=) this variable to be an empty string (String::new())
        */
        let mut guess = String::new();

        // print a line
        println!("Enter a guess:");

        /*
            using (io) from the standard library (std::io) we will read in the next line
            we read in the line to the variable 'guess' using (&mut)
            (&) refers to the fact that it is a REFERENCE, (&mut) means it is a mutable reference
            like variables, references are immutable by default so we need to specify that
            the reference (like the variable) is mutable. if we used (&guess) we would cause an error
        */
        io::stdin()
            /*
                we can break up long lines
                this will still be treat as a single statement by rust as that is delineated by the
                end of line symbol (;)
            */
            .read_line(&mut guess)
            .expect("Failed to read line.");

        /*
            converts the original string guess to an unsigned 32-bit integer - u32
            an unsigned integer cannot be a negative value
            a signed integer can be a negative value, i32 is the signed 32-bit integer
            integers can be i16, u16, i32, u32, i64, u64, i128 or u128
            this declaration "shadows" the originally declared guess variable
            allowing us to reuse the variable name

            switched from expect call to a match expression
        */
        let guess: u32 = match guess.trim().parse() { //.expect("Please enter a number!");
            Ok(num) => num,
            Err(_) => continue,
        };
        /* 
            trim removes any whitespace from the String - including any newline (\n) values
            this is important as one is created when we read the line previously
            parse lets us convert the string to a numeric value
         */

        /*
            print a line
            the placeholder ({}) inside the printed string allows us to insert a variable
            here we provide the (guess) variable to be printed as part of the string
        */
        println!("You guessed: {}", guess);

        /*
            calls the match expression when comparing (cmp) the variable of guess against secret_number
            the comparison is then put to an enum - Ordering. if the comparison matches any of the
            enum's conditions then the associated (println!) is run.
        */
        match guess.cmp(&secret_number) {
            // if guess is less than secret_number
            Ordering::Less => println!("You guessed too low!"),
            // if guess is greater than secret_number
            Ordering::Greater => println!("You guessed too high!"),
            // if guess is equal to secret_number
            Ordering::Equal => {
                println!("You got it!");
                break;
                // break out of the loop after the number is guessed
            },
            /*
                the match expression enumerates through each Ordering::...
                until it hit's one that returns a bool value of TRUE based on the
                comparison (cmp) conditions.
            */
        }
    }
}
