fn main() {
    let rec1 = Rectangle {
        width: 20,
        high: 10,
    };
    //De buging Structure println!("Rec1 is : {rec1:#?}");
    println!("The area of rectangle is {} pixel squares", area(&rec1));
    
    dbg!(&rec1);
}

#[derive(Debug)]
struct Rectangle  {
    width: u32,
    high: u32,
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width*rectangle.high
}