fn main() {
    //Basic of String!

    let mut s = String::from("This is S "); //creating new string

    println!("{s}");

    let data = "initial contents";
    let a = data.to_string(); //converting to string

    println!("{a}");
    
    //Updating String

    s.push_str("Bar"); //add Bar at the back of string
    println!("{s}");

    let mut s1 = String::from("foot");
    let s2 = "Bar";
    s1.push_str(s2);

    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let s3 = s1 + s2;

    println!("s3 is {s3}");

    let t1 = String::from("Tic");
    let t2 = String::from("Tac");
    let t3 = String::from("Toe");

    let full = format!("{t1}-{t2}-{t3}");
    println!("{full}");

    //Slicing Strings

    let hello = "Здравствуйте";
    let slice = &hello[0..4];
    println!("{slice}");

    //Iterating Over Strings

    for hello in "Зд".chars() {
        println!("{hello}");
    };

    for hello in "Зд".bytes() {
        println!("{hello}");
    };
}
