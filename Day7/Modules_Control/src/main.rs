use crate::garden::vegetables::Asparigus;

pub mod garden;

fn main() {
    let plant = Asparigus {};
    println!("I'm growing {:?}!", plant);
}
