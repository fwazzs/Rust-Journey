// shows borrowing: &s reads a value, &mut s lets a function change it
fn main() {

    //Borrowing (Immutable)

    let s1 = String::from("hello");

    // &s1 lends s1 to the function without giving up ownership
    let len = calculate_length(&s1); //borrow value of s1 (read only)

    println!("The length of '{s1}' is {len}.");

    //Borrowing (Muttable)
    let mut s = String::from("Yooseep");

    // &mut lends s so the function can change it; s must be declared mut
    change(&mut s); //borrow the s value and change it

    println!("{s}");
}

// reads the String through a reference and returns its length
fn calculate_length(s: &String) -> usize {
    s.len()
}

// changes the String it borrowed by adding text to the end
fn change (some_string: &mut String) {
    some_string.push_str(", UUHELL DUDE!");
}
