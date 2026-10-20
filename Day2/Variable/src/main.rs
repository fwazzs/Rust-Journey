fn main() {
    let mut x = 5; //mutable variable
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    //Constants

    const SECOND_IN_HOUR : u32 = 60 * 60; //different from immutable variable, constants must have a type annotation

    println!("The value of SECOND_IN_HOUR is: {SECOND_IN_HOUR}");

    //Shadowing

    let y = 5; //immutable variable
    let y = y + 1; //shadowing the previous variable

    println!("The value of y is: {y}");

    let y = y * 2; //shadowing again

    println!("The value of y is: {y}");

    let spaces = "   "; //immutable variable
    let spaces = spaces.len(); //shadowing the previous variable
    println!("The value of spaces is: {spaces}");
}
