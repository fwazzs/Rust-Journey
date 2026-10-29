// io lets us read input from the user
use std::io;

// shows arrays: a fixed-length group of values that all have the same type
fn main() {
    //Array (group of numbers)

    let a = [1, 2, 3, 4, 5];

    // access elements by index (starts at 0)
    let first = a[0];
    let second = a[1];

    println!("First is {first} and second is {second}");

    //

    let b = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    // mutable String to hold the typed index
    let mut index = String::new();

    // read a line from the keyboard into index
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // trim removes the newline, then parse turns the text into a usize
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // if the index is 5 or bigger the program panics, because the array only has 5 items
    let element = b[index];

    println!("The value of the element at index {index} is: {element}");

}
