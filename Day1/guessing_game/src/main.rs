use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!"); //print this

    let secret_number = rand::rng().random_range(1..=100); //generate random number between 1 and 100

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mutable variable (let mut)

        io::stdin()
            .read_line(&mut guess) //take input value of mut guess and store it in guess variable
            .expect("Failed to read line"); //expected of failed to read line

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //parse the input value to u32 and store it in guess variable

        println!("You guessed: {guess}"); //print the input

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
