use crate::account::list::User;

pub mod account;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Fawaz"),
        email: String::from("fawaz@example.com"),
        sign_in_count: 1,
    };
    println!("I'm growing {:?}!", user1);
}