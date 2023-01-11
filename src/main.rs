use rand::Rng;
use std::io::{self, Write};

mod tests;

const POSSIBLE_WORDS_TO_GUESS: &[&str] = &[
    "pineapple",
    "water",
    "milk",
    "pencil",
    "pen",
    "dog",
    "cat",
    "sofa",
    "javascript",
];
const WELCOME_MESSAGE: &'static str =
    "Welcome to the hangman guessing game mate, good luck or get hanged!";

fn main() {
    println!("{WELCOME_MESSAGE}");
    start_game(POSSIBLE_WORDS_TO_GUESS)
}

fn start_game(words: &[&str]) {
    let mut maximum_player_lives: i8 = 6;
    println!("Remaining lives: {maximum_player_lives}");

    let random_generated_number = rand::thread_rng().gen_range(0..=words.len() - 1);
    let word_to_be_guessed = words[random_generated_number];

    let mut counter = 0;

    let mut underscored_word_to_guess: Vec<String> = Vec::new();

    loop {
        if counter == word_to_be_guessed.len() {
            break;
        }

        underscored_word_to_guess.push("_".to_string());
        counter += 1;
    }

    io::stdout().write(b"Start guessing:").expect("Failed");

    while underscored_word_to_guess.contains(&"_".to_string()) && maximum_player_lives > 0 {
        println!("Remaining lifes: {:?} ", maximum_player_lives);

        println!("{:?}", underscored_word_to_guess);

        let mut user_input_letter = String::new();

        io::stdin()
            .read_line(&mut user_input_letter)
            .expect("Failed reading line");

        let user_guess = user_input_letter.chars().next().unwrap();

        if word_to_be_guessed.contains(user_guess) {
            for (index, value) in word_to_be_guessed.match_indices(user_guess) {
                underscored_word_to_guess[index] = value.to_string();
            }
        } else {
            maximum_player_lives -= 1;
        }
    }

    if maximum_player_lives == 0 {
        println!("YOU ARE HANGED!");
        return;
    }

    println!("YOU WON!!!");
}
