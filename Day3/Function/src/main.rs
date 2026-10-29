// shows functions: calling them, parameters, return values and expressions
fn main() {
    println!("Hello, world!");

    //Call Function
    // another() has no parameters and returns nothing
    another();
    // add_one(32) passes 32 as x and returns 33
    let x = add_one(32);

    //Expression

    // a block {} is an expression: its last line (without ;) is the value
    let y = {
        let x = 3;
        x+3
    };
    println!("The result of add one : {x}");

    println!("The Expression result : {y}");
}

// a function with no parameters and no return value
fn another() {
    println!("This is another function");
}

//Parameter

// x: i32 is a parameter with a type; -> i32 is the return type
fn add_one(x: i32) -> i32 {
    // no semicolon, so this value is returned
    x + 1
}
