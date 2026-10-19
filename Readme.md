## WELCOME TO THE RUST LEARNING LOG
It would be contain all of my Rust Programming Language learning progress. It started on 19 Oct 2026.

### First Week
## Day 1 (19 Oct 2026)

In this day, I had installed Rust on my computer and do little bit experiment with it by writing some basic code and others :
1. Learn about Cargo (cargo build, cargo run, cargo build --release, etc)
2. Varibale (var = immutatable & var mut = mutable variable)
3. std::io (Input and Output library)

    io::stdin()
        .read_line(&mut guess) //take input value of mut guess and store it in guess variable
        .expect("Failed to read line");

4. std::cmp::Ordering; (Comparation library)

    match guess.cmp(&secret_number) { //compare guess with secret number
            Ordering::Less => println!("Too small!"), //if guess is less than secret number
            Ordering::Greater => println!("Too big!"), //if guess is greater than secret number
            Ordering::Equal => {
                println!("You win!");
                break;
            } //if guess is equal to secret number then break the Loop
        }

5. use rand::prelude::*; (Random Generating Number)

        rand::rng().random_range(1..=100); //generate random number between 1 and 100