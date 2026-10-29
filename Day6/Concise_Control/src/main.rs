fn main() {
    // Option<u8> holding the value 3
    let config_max = Some(3u8);

    // match checks every arm: Some(max) runs when there is a value, _ catches None (does nothing)
    match config_max {
        Some(max) => println!("The maximum of config {max}"),
        _ => ()
    }

    //simple way to write code above is below here

    // if let is shorter when we only care about one pattern (Some here) and ignore the rest
    if let Some(max) = config_max {
        println!("The maximum of config {max}")
    }

    //if let ... else
    // create a Quarter coin from Alaska
    let coin = Coin::Quarter(UsState::Alaska);
    // counter that only goes up when the coin is NOT a quarter
    let mut count = 0;

    // if the coin matches Quarter, bind the inner UsState to `state` and print it
    if let Coin::Quarter(state) = coin {
        println!("State of Queater is {state:?}");
    } else {
        // any other coin: the else branch runs
        count += 1;
    }
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

// returns the value of a coin in cents; the match must cover every Coin variant
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // bind the UsState inside the Quarter, print it, then return 25 (the last expression in the block)
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}!");
            25
        },
    }
}
