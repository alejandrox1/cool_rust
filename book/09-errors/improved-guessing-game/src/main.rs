extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guessed value must be between 1 and 100. Got: {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value as u32
    }
}

fn main() {
    println!("Guess number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("What's your guess? ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess: Guess = match guess.trim().parse() {
            Ok(value) => Guess::new(value),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value);
        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Perfect!");
                break;
            }
        }
    }
}
