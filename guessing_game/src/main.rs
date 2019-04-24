use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!, secret was {}", secret),
        Ordering::Greater => println!("Too big!, secret was {}", secret),
        Ordering::Equal => println!("You win!"),
    }
}
