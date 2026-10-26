fn main() {

    //Borrowing (Immutable)

    let s1 = String::from("hello");

    let len = calculate_length(&s1); //borrow value of s1 (read only)

    println!("The length of '{s1}' is {len}.");

    //Borrowing (Muttable)
    let mut s = String::from("Yooseep");

    change(&mut s); //borrow the s value and change it

    println!("{s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change (some_string: &mut String) {
    some_string.push_str(", UUHELL DUDE!");
}