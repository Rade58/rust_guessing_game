// use std::io;

use rand::Rng;

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

    println!("You guessed: {guess}");
}
