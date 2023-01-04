mod addition;
use rand::Rng;
use std::io;

fn main() {
    let mut maximum_player_lives: i8 = 6;

    const POSSIBLE_WORDS_TO_GUESS: &[&str] = &["pineapple", "water", "milk"];

    const WELCOME_MESSAGE: &'static str =
        "Welcome to the hangman guessing game mate, good luck or get hanged!";

    println!("{WELCOME_MESSAGE}");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("User input is: {user_input }");

    start_game(POSSIBLE_WORDS_TO_GUESS)
}

fn start_game(words: &[&str]) {
    let random_generated_number = rand::thread_rng().gen_range(0..=words.len() - 1);

    println!("Provided array {:?}:", words);
    println!("Random number is: {random_generated_number}");
}
