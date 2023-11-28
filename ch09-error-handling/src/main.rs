use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};
use std::mem;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() -> Result<(), Error> {
    let first_guess = Guess::new(102);
    println!("{}", first_guess.value);

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let fileCreateHandle = |error: Error| {
        panic!("Problem creating the file: {:?}", error);
    };
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(fileCreateHandle)
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting = "hello".to_string();

    let greeting_fn = || {
        println!("{}", greeting);
        mem::drop(greeting);
    };

    greeting_fn();

    let mut username = String::new();
    let mut greeting_file: File = File::open("hello.txt")?;
    println!("");
    greeting_file.read_to_string(&mut username)?;
    println!("{}", username);
    Ok(())
}
