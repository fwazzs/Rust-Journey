// shows methods: functions attached to a struct with an impl block
fn main() {
    // three rectangles to compare
    let rec1 = Rectangle {
        width: 32,
        height: 10,
    };

    let rec2 = Rectangle {
        width: 31,
        height: 9,
    };

    let rec3 = Rectangle {
        width: 33,
        height: 33,
    };

    // call the method with dot syntax: rec1.area() is the same as Rectangle::area(&rec1)
    println!("The area of Rectangle is : {}", rec1.area());

    // can_hold returns true only if both width and height of rec1 are bigger than the other
    println!("Does the first rectangle can hold second rectangle ? {}", rec1.can_hold(&rec2));
    println!("Does the first rectangle can hold third rectangle ? {}", rec1.can_hold(&rec3));

}

// a struct groups the width and height
struct Rectangle {
    width: u32,
    height: u32,
}

// impl holds the functions that belong to Rectangle
impl Rectangle {
    // &self borrows the rectangle the method is called on
    fn area (&self) -> u32 {
        self.width * self.height
    }

    // takes a second rectangle to compare with
    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
