// shows let, let mut, const and shadowing
fn main() {
    let mut x = 5; //mutable variable: can be changed later
    println!("The value of x is: {x}");

    x = 6; // allowed because x is mut
    println!("The value of x is: {x}");

    //Constants

    // constants are always immutable and must have a type annotation (u32 here)
    const SECOND_IN_HOUR : u32 = 60 * 60; //different from immutable variable, constants must have a type annotation

    println!("The value of SECOND_IN_HOUR is: {SECOND_IN_HOUR}");

    //Shadowing

    let y = 5; //immutable variable
    let y = y + 1; //shadowing the previous variable: a new y is created with the value 6

    println!("The value of y is: {y}");

    let y = y * 2; //shadowing again: a new y is created with the value 12

    println!("The value of y is: {y}");

    let spaces = "   "; //immutable variable (a &str)
    let spaces = spaces.len(); //shadowing the previous variable: the type changes from &str to usize
    println!("The value of spaces is: {spaces}");
}
