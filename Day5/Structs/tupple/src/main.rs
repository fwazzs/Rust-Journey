// shows tuple structs: named types made of values without field names
fn main() {
    // Color and Point are different types even though they both hold three i32s
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

// tuple struct: fields are accessed by index (.0, .1, .2), not by name
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
