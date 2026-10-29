// shows match: running different code for each variant of an enum
fn main() {
    // pass a Quarter from Alaska and print the value it returns
    let x = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Nilai sennya : {x}");
}

// the kinds of coins; only Quarter carries extra data (which state it came from)
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Debug lets us print the state with {:?}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

// returns the value of the coin in cents; every variant must be handled
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // bind the UsState inside the Quarter, print it, then return 25
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}!");
            25
        },
    }
}
