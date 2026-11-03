fn main() {
    let row = vec![
        Spreadsheetcell::Int(32),
        Spreadsheetcell::Float(19.23),
        Spreadsheetcell::Text(String::from("Hello There!")),
    ];

    println!("{:?}", row);
}

#[derive(Debug)]
enum Spreadsheetcell {
    Int(i32),
    Float(f64),
    Text(String),
}