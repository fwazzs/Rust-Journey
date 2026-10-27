fn main() {
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

    println!("The area of Rectangle is : {}", rec1.area());

    println!("Does the first rectangle can hold second rectangle ? {}", rec1.can_hold(&rec2));
    println!("Does the first rectangle can hold third rectangle ? {}", rec1.can_hold(&rec3));
    
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}