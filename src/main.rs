// use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Welcome to 'Guess the number game'!");

    //
    let secret_number = rand::thread_rng().gen_range(1..=100); //

    println!("secret number: {secret_number}");

    loop {
        println!("Input your guess!");
        let mut guess = String::new();

        // STANDARD INPUT PROMPT
        // io::stdin()
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        // I DON'T WANT EXPECT, BECAUSE expect CRASHES PROGRAM
        // WITH A MESSAGE YOU PROVIDED
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // I WANT ERROR HANDLING WHEREE LOOP CONTINUES AFTER
        // INVALID NON-u32 INPUT

        // parse returns enum Result we can match
        let guess: u32 = match guess.trim().parse() {
            Result::Ok(num) => num,
            Result::Err(_) => continue,
        };

        // match guess.cmp(&secret_number.to_string()) {
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed it! You won!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Less => {
                println!("Too small!");
            }
        }
    }

    // println!("You guessed: {guess}");
}
