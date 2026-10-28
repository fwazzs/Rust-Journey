fn main () {

    let home = IpAddr::V4(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));

    let mut msg = Message::Quit;
    msg = Message::Move { x: 32, y: 12 };
    msg = Message::Write(String::from("Hello World"));
    msg = Message::Change_color(1, 1, 1);
}

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x:u32 , y:i32},
    Write(String),
    Change_color(i32,i32,i32),
}
