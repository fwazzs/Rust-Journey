// shows a basic struct: create an instance and change one field
fn main() {
    // create a User; mut is needed so we can change a field later
    let mut user1 = User {
        active: true,
        username: String::from("Fawaz"),
        email: String::from("Fawaz123@gmail.com"),
        sign_in_count: 1,
    };

    // change the email field with dot syntax (the whole instance is mut)
    user1.email = String::from("Pewes123@gmail.com");
}

// a struct groups related values under named fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
