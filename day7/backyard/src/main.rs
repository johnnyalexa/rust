
use crate::garden::vegetables::Aspagarus;

pub mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Aspagarus{};
    println!("I'm growing {:?}!", plant);
}
