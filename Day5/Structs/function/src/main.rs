fn main() {
    let budi = build_user ("Budi Kurno".to_string(), "Budi123@gmail.com".to_string());

    let user2 = User {
        active: budi.active,
        email: String::from("user2@gmail.com"),
        username: String::from("user2"),
        sign_in_count: budi.sign_in_count,
    };

    let user3 = User {
        email: String::from("user3@gmail.com");
        ..user2
    }
}

fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}