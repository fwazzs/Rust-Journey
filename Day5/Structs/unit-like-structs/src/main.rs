// shows a unit-like struct: a struct with no fields
fn main() {
    // no data to store, but it is still its own type
    let subject = AlwaysEqual;
}

// unit-like struct: useful when you need a type but no data (e.g. for a trait later)
struct AlwaysEqual;
