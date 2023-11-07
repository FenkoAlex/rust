use rand::Rng;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Penny);
    println!("{}", value_in_cents(Coin::Dime));
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {}", six.unwrap());
    println!("unwrap none: {}", none.unwrap_or_default());
    println!("none is None? - {}", none == None);

    let a: Option<i32> = None;

    if a == None {
        println!("a == None");
    } else {
        println!("a != None");
    }

    game();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn game() {
    let dice_roll = rand::thread_rng().gen_range(1..=10);
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        number => reroll(number),
    }

    fn add_fancy_hat() {
        println!("You got fancy hat!");
    }
    fn remove_fancy_hat() {
        println!("You're losing your fancy hat!");
    }
    fn reroll(number: i32) {
        println!("{} - reroll", number);
        game();
    }
}
