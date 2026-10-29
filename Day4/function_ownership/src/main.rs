// shows ownership: a String is moved into a function, a number is copied
fn main() {
    let x = String::from ("Hello World");

    // x is moved into take_ownership, so x cannot be used after this line
    take_ownership(x); //variable x will only available until this line

    //println!("{x}"); //if we run this line, the code will get error message

    let y = 5;

    // y is copied (i32 is a simple stack value), so y can still be used after this line
    make_copy(y); //variable y will only available until this line

    //println!("{y}"); //if we run this line, the code will get error message
}

// takes ownership of the String; it is dropped at the end of this function
fn take_ownership (some_string: String) {
    println!("{some_string}");
}

// takes a copy of the number
fn make_copy (some_int: i32) {
    println!("{some_int}");
}

//Flow
//Main create x variable then move it to take_ownership function. after we decleared the function, variable x won't can be use again
//Main create y variable then move it to make_copy function. After we decleared the function, variable y wont' can be use again
