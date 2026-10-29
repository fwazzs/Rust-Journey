// shows passing a struct to a function by reference and printing it with Debug
fn main() {
    // create a Rectangle with width 20 and height (named "high" here) 10
    let rec1 = Rectangle {
        width: 20,
        high: 10,
    };
    //De buging Structure println!("Rec1 is : {rec1:#?}");
    // &rec1 borrows the rectangle so area can read it without taking ownership
    println!("The area of rectangle is {} pixel squares", area(&rec1));

    // dbg! prints the file, line and value of rec1 to stderr (debug output)
    dbg!(&rec1);
}

// #[derive(Debug)] lets the struct be printed with {:?} or dbg!
#[derive(Debug)]
struct Rectangle  {
    width: u32,
    high: u32,
}

// takes a reference to a Rectangle and returns width * high
fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width*rectangle.high
}
