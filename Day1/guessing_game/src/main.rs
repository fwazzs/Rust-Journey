use std::io;

fn main() {
    println!("Guesse the number!"); //print this
    println!("Please input your guess.");

    let mut guess = String::new(); //mutable variable (let mut)

    io::stdin()
        .read_line(&mut guess) //take input value of mut guess and store it in guess variable
        .expect("Failed to read line");

    println!("You guessed: {guess}"); //print the input
}
