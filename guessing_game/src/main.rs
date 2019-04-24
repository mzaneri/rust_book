use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 100);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess_num: u32 = guess.trim().parse().expect("Please type a number!");

        // println!("You guessed: {}", guess);

        match guess_num.cmp(&secret) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
