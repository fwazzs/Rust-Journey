use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // unwrap : pesan panic bawaan
    let f = File::open("hello.txt").unwrap();

    // expect : kita tulis pesan sendiri
    let f = File::open("hello.txt")
        .expect("hello.txt should be included in this program");
}
