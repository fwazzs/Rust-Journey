fn main () {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //reading element of vector

    let x = vec![1,2,3,4,5,6,7];

    let third = &x[2]; //It would be error if the value out of the range
    println!("The third element of vector is {}",third);

    let nine: Option<&i32> = x.get(10); //if the value out of the range, it would return None
    match nine {
        Some(nine) => println!("The Nine element of vector is {}",third),
        None => println!("There's no element in the Nine of vector"),
    }
}