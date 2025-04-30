use std::io::stdin;
use crate::get_input;
use rand::Rng;

pub fn guessing_game() {
    println!("Guess a number:");

    let guess: i32 = get_input!().trim().parse().expect("You are using the wrong numba bish");

    println!("You guessed {}", guess);

    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", random_number);
    match guess {
        1 => {
            println!("You guessed a too small numba bish.");
        }, 
        _ => println!("wrong and everything else.")
    }
}
