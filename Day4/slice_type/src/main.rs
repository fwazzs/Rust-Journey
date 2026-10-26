fn main() {
    let x = String::from("someone has to relized");

    let word = first_word(&x);

    println!("{word}");

    //String Slice

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];

    println!("{slice}");

    let slice = &s[..];

    println!("{slice}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

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

