fn main() {
    let number = 3;

    //single condition
    if number < 5 {
        println!("Your number is below 5");
    } else {
        println!("Your number is above 5");
    }

    //mutltiple condition

    let nomor = 6;

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
    let numb = if condition {5} else {6};

    println!{"The value of number is : {numb}"};
}
