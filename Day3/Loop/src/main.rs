fn main() {
    // DON'T DELETE THE COMMENT IN THIS CODE, IT WOULD CRASH YOUR POTATO COMPUTER LMAOO!!!
    //loop {
    //    println!("Hello Dump!");
    //}

    let mut x = 0;

    let result = loop {
        x += 1;

        if x == 10 {
            break x;
        } else {
            println!("x is {x}");
        }
    };

    println!("The result is {result}");


    //Disambiguating with Loop Lables

    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } else if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");


    //While

    let mut angka = 3;

    while angka != 0 {
        println!("{angka}");

        angka -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10,20,30,40,50];

    let mut index = 0;

    while index < 5 {
        println!("The value is : {}", a[index]);

        index += 1;
    }

    //for
    println!("
    
    ");

    for element in a {
        println!("the value is : {element}");
    }

    //
    println!("
    
    ");

    for c in 1..4 {
        println!("{c}!");
    }
}
