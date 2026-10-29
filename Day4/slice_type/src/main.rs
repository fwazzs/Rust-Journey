// shows slices: a reference to part of a String
fn main() {
    let x = String::from("someone has to relized");

    // first_word returns the index of the first space (a usize, not the word itself yet)
    let word = first_word(&x);

    println!("{word}");

    //String Slice

    let s = String::from("hello");

    let len = s.len();

    // &s[3..len] takes from index 3 up to (not including) len, so "lo"
    let slice = &s[3..len];

    println!("{slice}");

    // &s[..] takes the whole string
    let slice = &s[..];

    println!("{slice}");
}

// finds the index of the first space in the string; if there is none, returns the length
fn first_word(s: &String) -> usize {
    // turn the String into bytes so each character can be compared with b' ' (a space byte)
    let bytes = s.as_bytes();

    // enumerate gives both the index i and the byte item for each loop
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("Space on {i}");
            return i;
        } else {
            println!("{i}");
        }
    }

    s.len()
}

