// shows the two string types: &str (fixed) and String (growable)
fn main() {
    // &str: a string literal written in the code, cannot be changed
    let x = "YOO BRO WHAT'S SUPPP!!!"; //&str String literal that decleared by dev and immutable

    // String: stored on the heap, can grow and be changed because it is mut
    let mut y = String::from("Hello Fella!"); //String that able decleared either dev or user (mutable)
    y.push_str(", My big boy!"); //push "..." to y variable

    println!("{y}");
}
