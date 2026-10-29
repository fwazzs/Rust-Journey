// shows the three kinds of loops: loop, while and for
fn main() {
    // DON'T DELETE THE COMMENT IN THIS CODE, IT WOULD CRASH YOUR POTATO COMPUTER LMAOO!!!
    //loop {
    //    println!("Hello Dump!");
    //}

    let mut x = 0;

    // loop runs forever until break; break x gives x back as the value of result
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
    // 'counting_up is a label, so the inner break can stop this outer loop
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        // inner loop: break only leaves this loop, break 'counting_up leaves the outer loop
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

    // while runs as long as the condition is true
    while angka != 0 {
        println!("{angka}");

        angka -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10,20,30,40,50];

    let mut index = 0;

    // while loop that reads each element by index
    while index < 5 {
        println!("The value is : {}", a[index]);

        index += 1;
    }

    //for
    println!("

    ");

    // for goes through every element of the array directly (no index needed)
    for element in a {
        println!("the value is : {element}");
    }

    //
    println!("

    ");

    // 1..4 is a range: it gives 1, 2, 3 (4 is not included)
    for c in 1..4 {
        println!("{c}!");
    }
}
