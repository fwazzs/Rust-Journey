// shows building structs with a function, field init shorthand and struct update syntax
fn main() {
    // build_user returns a User built from the two Strings
    let budi = build_user ("Budi Kurno".to_string(), "Budi123@gmail.com".to_string());

    // create user2 by copying each field from budi
    let user2 = User {
        active: budi.active,
        email: String::from("user2@gmail.com"),
        username: String::from("user2"),
        sign_in_count: budi.sign_in_count,
    };

    // NOTE: this does not compile: the field line ends with ';' but it should end with ','
    // and the struct update syntax (..user2) should not be on its own line after a ';'
    let user3 = User {
        email: String::from("user3@gmail.com");
        ..user2
    }
}

// builds a User; email and username use field init shorthand (same name as the field)
fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

// a struct groups related values under named fields
struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}
