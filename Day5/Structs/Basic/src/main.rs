fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Fawaz"),
        email: String::from("Fawaz123@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("Pewes123@gmail.com");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}