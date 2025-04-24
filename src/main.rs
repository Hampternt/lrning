use std::io::stdin;

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

// Starts the program.
fn main() {
    println!("Hello, world!");
    println!("having fun");
    print_words_given_to_it("Something testington.");

    let wellthisinice = get_input!();

    println!("Now print it.");
    print_words_given_to_it(&wellthisinice);
}

//Prints the values of given arguments.
fn print_words_given_to_it(words: &str){
    println!("{}", words);
}
