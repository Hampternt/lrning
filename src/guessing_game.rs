use std::io::stdin;
use crate::get_input;
use rand;

pub fn guessing_game() {
    println!("Guess a number:");

    let guess = get_input!();

    println!("You guessed {}", guess);
}
