fn main() {
    let a = Some(5);
    let b = Plus_One(a);
    let c = Plus_One(None);
}

fn Plus_One (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
