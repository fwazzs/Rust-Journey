// shows matching an Option with match
fn main() {
    // a has Some(5)
    let a = Some(5);
    // Plus_One adds one to the value inside Some, so b is Some(6)
    let b = Plus_One(a);
    // None stays None
    let c = Plus_One(None);
}

// match handles both cases: None returns None, Some(i) returns Some with i + 1
fn Plus_One (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
