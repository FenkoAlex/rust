use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn strTest() {
    // в чем разница &str и Stering
    let test = "test";
    let mut test2 = String::new();
    test2 = "test2".to_string();
    println!("{test}, {}, {}", test2, 2);

    match 5.cmp(&10) {
        //почему тут значение надо передавать по ссылке? или тут передается само значение, поэтому ссылка?
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("--");
        println!("{guess}"); // почему он конкатинирует строки?) если бы я не затенял guess, получил бы многострочную переменную
                             // что происходит с затененными guess, они торчат в памяти?
        println!("--");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
