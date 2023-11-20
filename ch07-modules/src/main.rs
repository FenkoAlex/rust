use crate::garden::vegetables::Asparagus;
use front_of_house::serving;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    let plant2 = garden::vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);
}
