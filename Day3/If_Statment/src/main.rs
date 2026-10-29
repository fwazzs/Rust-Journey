// shows if, else if, else and using if inside let
fn main() {
    let number = 3;

    //single condition
    // only one branch runs: the first condition that is true
    if number < 5 {
        println!("Your number is below 5");
    } else {
        println!("Your number is above 5");
    }

    //mutltiple condition

    let nomor = 6;

    // checks run from top to bottom; the first true one runs and the rest are skipped
    if nomor % 4 == 0 {
        println!("Number is divisible by 4");
    } else if nomor % 3 == 0 {
        println!("Number is divisible by 3");
    } else if nomor % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4,3,2");
    }

    //if statment in let
    let condition = true;
    // if is an expression, so its result can be stored; both branches must return the same type (i32 here)
    let numb = if condition {5} else {6};

    println!{"The value of number is : {numb}"};
}
