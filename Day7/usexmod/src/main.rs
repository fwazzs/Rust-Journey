use crate::foh::front_of_house::hosting; //shorthand for the path to hosting

pub mod foh;

fn main () {
    hosting::add_to_waitlist(); 
}