use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game!");

    let number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        print!("Guess a number [1, 100]: ");
        io::stdout()
            .flush()
            .unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if guess < 1 || guess > 100 {
            continue;
        }
        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
