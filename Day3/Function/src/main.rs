fn main() {
    println!("Hello, world!");

    //Call Function
    another();
    let x = add_one(32);

    //Expression 

    let y = {
        let x = 3;
        x+3
    };
    println!("The result of add one : {x}");

    println!("The Expression result : {y}");
}

fn another() {
    println!("This is another function");
}

//Parameter

fn add_one(x: i32) -> i32 {
    x + 1
}
