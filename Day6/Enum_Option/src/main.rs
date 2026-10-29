// shows Option: a value that is either Some(value) or None
fn main() {
    // x and y hold Some with a value; the type is inferred as Option<i32>
    let x = Some(5);
    let y = Some(8);

    // z has no value, so the type must be written out
    let z:Option<u32> = None;
}
