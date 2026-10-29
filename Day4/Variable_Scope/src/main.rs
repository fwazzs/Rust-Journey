// shows variable scope: a variable only exists inside the block it is declared in
fn main() { //s unvalid (not yet decleared)

    let s = "Hello There!"; //s valid from this

    //s still valid in this area
}// s no longer valid (out of scope)
