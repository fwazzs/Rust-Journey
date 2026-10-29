// shows enums: a type that can be one of several variants
fn main () {

    // V4 holds four u8 numbers, so it is an IPv4 address
    let home = IpAddr::V4(127,0,0,1);

    // V6 holds a String, so it is an IPv6 address
    let loopback = IpAddr::V6(String::from("::1"));

    // msg can be any variant of Message; it is reassigned several times to show each kind
    let mut msg = Message::Quit; // no data
    msg = Message::Move { x: 32, y: 12 }; // named fields like a struct
    msg = Message::Write(String::from("Hello World")); // one String
    msg = Message::Change_color(1, 1, 1); // three i32 values
}

// each variant can hold a different kind of data
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

// variants with no data, a named-field set, a String, or a tuple of values
enum Message {
    Quit,
    Move {x:u32 , y:i32},
    Write(String),
    Change_color(i32,i32,i32),
}
