use std::io::stdin;
use crate::get_input;
use rand::Rng;

pub fn guessing_game() {
    // Generates a random number between the 1-100 range.
    let random_number: i32 = rand::thread_rng().gen_range(1..=100);
    
    //Prints the number that is the random number and gives user feedbak that they worked.
    println!("You guessed the number: {}", random_number);

    //Initiates the loop for the guessing game.
    guessing_game_loop(random_number);
}

fn guessing_game_loop(random_number:i32){

    // creates a while loop condition breaker.
    let mut break_condition: bool = true;

    // starts while loop that runs until the user can guess the corect number.
    while break_condition {
        println!("Make your guess: any numba between 1 and 100");

        // Asks the user to guess a number between 1-100 should maybe verify the number is between
        // that range before continuin but i dont care enough to verfiy that at the moment.
        let guess: i32 = get_input!().trim().parse().expect("You are using the wrong numba bish");
     
        match guess {
            guessed_number if guessed_number > random_number => {
                println!("You guessed a too big numba bish.");
            },
            guessed_number if guessed_number < random_number => {
                println!("you guess is too small ye ye ye.");
            },
            guessed_number if guessed_number == random_number => {
                println!("Yay the answer is the same ye ha ha :3");
                break_condition = false;
            },
            _ => {
                println!("wrong and everything else.");
                break_condition = false;
            },
        };
    }
}
