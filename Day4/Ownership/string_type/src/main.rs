fn main() {
    let x = "YOO BRO WHAT'S SUPPP!!!"; //&str String literal that decleared by dev and immutable

    let mut y = String::from("Hello Fella!"); //String that able decleared either dev or user (mutable)
    y.push_str(", My big boy!"); //push "..." to y variable

    println!("{y}");
}
