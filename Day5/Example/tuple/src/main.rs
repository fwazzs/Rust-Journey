fn main() {
    let rec1 = (20,10);

    println!("The area of rectangle is {} pixel squares",area(rec1));
}

fn area (dimention: (u32, u32)) -> u32 {
    dimention.0*dimention.1
}
