use std::io::stdin;
mod guessing_game;

enum Choices {
    NumberGuessingGame,
    EcoGame, 
    Other
}

// Starts the program.
fn main() {
    println!("What function do you want to use?:");
    let choice:Choices = return_right_option();

    match choice{
        Choices::NumberGuessingGame => guessing_game::guessing_game(),
        Choices::Other => println!("You chose other good job"),
        Choices::EcoGame => println!("need to implement this later :3")
    }
}

#[macro_export]
macro_rules! get_input {
    () => {
        {
        let mut input = String::new();
        println!("Write input:");
        stdin().read_line(&mut input).expect("Failed to read line.");
        input
        }
    }
}

fn return_right_option() -> Choices {
    println!("Running return right_option_function");
    let function_names = ["Number Guessing Game.", "Print my word thing."];

    println!("Here are the options choose one.");
    for (index, item) in function_names.iter().enumerate() {
        println!("{}. {}",index + 1, item);         
    }
    
    //This gets the user input from the console.
    let choice_string = get_input!();
    //This converts the user input from a String to a i32 number which is needed for the choices
    //match.
    let choice_num: i32 = choice_string.trim().parse().expect("Please enter a valid number");

    //The choices Match, figures out which choice the user wants to run with and returns the
    //choices value which then goes to Main. 
    match choice_num {
        1 => {
            println!("You chose option 1, Number Guessing Game");
            return Choices::NumberGuessingGame; 
        },
        2 => {
            println!("You chose option 2, EcoGame");
            return Choices::EcoGame;
        },
        _ => {
            println!("You have chosen a non valid option");
            return Choices::Other;
        },
    }
}



//Prints the values of given arguments.
fn print_words_given_to_it(words: &str){
    println!("{}", words);
}
