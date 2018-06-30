extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Number guessing game");
    println!("Enter a number between 1 - 10");
    let mut guessed_number = String::new();
    let lucky_number : u32 = rand::thread_rng().gen_range(1, 10);

    io::stdin()
        .read_line(&mut guessed_number)
        .expect("Failed to read the name");

    let number: u32 = guessed_number.trim().parse().expect("Not a number");

    if lucky_number == number  {
        println!("You guessed the right number {}", number)
    } else {
        println!("Sorry, the lucky number is {}", lucky_number)
    }
}
