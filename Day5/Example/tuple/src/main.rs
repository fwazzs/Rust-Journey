// shows calculating an area with a tuple (the names of width and height are lost)
fn main() {
    // width is 20 and height is 10 stored as a tuple
    let rec1 = (20,10);

    println!("The area of rectangle is {} pixel squares",area(rec1));
}

// takes the tuple and multiplies its two values; .0 is width and .1 is height
fn area (dimention: (u32, u32)) -> u32 {
    dimention.0*dimention.1
}
