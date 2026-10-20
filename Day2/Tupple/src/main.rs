fn main() {
    //Tupple (group of number)

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {y}");
    println!("The value of x is {x}");
    println!("The value of z is {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{five_hundred}");
    println!("{six_point_four}");
    println!("{one}");
}
