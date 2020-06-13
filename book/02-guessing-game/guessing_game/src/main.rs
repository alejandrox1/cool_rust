use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the right number :-)\n");

    let secret_number = rand::thread_rng().gen_range(0, 1001);

    loop {
        println!("What's your guess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("expected guess to be a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        };
    }
}
