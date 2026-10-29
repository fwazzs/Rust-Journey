// Ordering is used to compare two values; io lets us read input from the user
use std::cmp::Ordering;
use std::io;
// rand gives us random numbers (the prelude brings the common traits in)
use rand::prelude::*;

// a guessing game: the player keeps guessing until they match the secret number
fn main() {
    println!("Guess the number!"); //print this

    // generate random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    // loop repeats until the player guesses correctly (break)
    loop {
        println!("Please input your guess.");

        // mutable String to hold what the user types (let mut)
        let mut guess = String::new();

        // read a line from the keyboard into guess
        io::stdin()
            .read_line(&mut guess) //take input value of mut guess and store it in guess variable
            .expect("Failed to read line"); //expected of failed to read line

        // trim removes the newline, parse turns the text into a u32
        // if it is not a number (Err), skip this round and ask again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //parse the input value to u32 and store it in guess variable

        println!("You guessed: {guess}"); //print the input

        // compare the guess with the secret number
        match guess.cmp(&secret_number) { //compare guess with secret number
            Ordering::Less => println!("Too small!"), //if guess is less than secret number
            Ordering::Greater => println!("Too big!"), //if guess is greater than secret number
            Ordering::Equal => {
                println!("You win!");
                break;
            } //if guess is equal to secret number then break the Loop
        }
    }
    println!("The secret number is: {secret_number}"); //print the secret number
}
