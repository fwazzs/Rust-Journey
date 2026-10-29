// shows tuples: a fixed-length group of values that can have different types
fn main() {
    //Tupple (group of number)

    // the tuple holds an i32, an f64 and a u8
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring: split the tuple into three separate variables
    let (x, y, z) = tup;

    println!("The value of y is {y}");
    println!("The value of x is {x}");
    println!("The value of z is {z}");

    // access each value by its index with a dot
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{five_hundred}");
    println!("{six_point_four}");
    println!("{one}");
}
