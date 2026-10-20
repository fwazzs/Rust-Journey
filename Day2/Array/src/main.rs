use std::io;

fn main() {
    //Array (group of strings)

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("First is {first} and second is {second}");

    //

    let b = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = b[index];

    println!("The value of the element at index {index} is: {element}");

}
