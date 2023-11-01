use std::io;

fn return5(x: bool) -> i32 {
    if x {
        5
    } else {
        6
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let y = {
        let x = 3;
        x;
    };
    print_type_of(&y);
    print_type_of(&return5);
    let x = return5(true);
    let x2 = return5(false);

    println!("{x}, {x2}");

    let mut number = 3;

    let arr = 1..4;

    for number in 1..4 {
        println!("{number}!");
    }
}