use std::io;

fn guessing_game() {
    println!("Guess a number:");

    let mut guess = get_input!();

    println!("You guessed {}", guess);
}
