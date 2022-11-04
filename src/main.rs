// use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Welcome to 'Guess the number game'!");

    //
    let secret_number = rand::thread_rng().gen_range(1..=100); //

    println!("secret number: {secret_number}");

    println!("Input your guess!");

    let mut guess = String::new();

    // io::stdin()
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // match guess.cmp(&secret_number.to_string()) {
    match guess.cmp(&secret_number) {
        Ordering::Equal => {
            println!("You guessed it! You won!")
        }
        Ordering::Greater => {
            println!("Too big!")
        }
        Ordering::Less => {
            println!("Too small!")
        }
    }

    println!("You guessed: {guess}");
}
