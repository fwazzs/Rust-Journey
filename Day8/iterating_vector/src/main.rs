fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    println!("");

    let mut x = vec![100, 32, 57];
    for i in &mut x {
        *i += 50;
        println!("{i}");
    }
}
