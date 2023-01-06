use rand::Rng;
use std::io::{self, Write};

mod tests;
fn main() {
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

    println!("{WELCOME_MESSAGE}");

    start_game(POSSIBLE_WORDS_TO_GUESS)
}

fn start_game(words: &[&str]) {
    let mut maximum_player_lives: i8 = 6;
    println!("Remaining lives: {maximum_player_lives}");

    let random_generated_number = rand::thread_rng().gen_range(0..=words.len() - 1);
    let word_to_be_guessed = words[random_generated_number];

    let mut counter = 0;

    let mut underscore_word: Vec<String> = Vec::new();

    loop {
        if counter == word_to_be_guessed.len() {
            break;
        }

        underscore_word.push("_".to_string());
        counter += 1;
    }

    io::stdout().write(b"Start guessing:").expect("Failed");

    while underscore_word.contains(&"_".to_string()) && maximum_player_lives > 0 {
        println!("Remaining lifes: {:?} ", maximum_player_lives);

        println!("{:?}", underscore_word);

        let mut user_input_letter = String::new();

        io::stdin()
            .read_line(&mut user_input_letter)
            .expect("Failed reading line");

        let is_letter_in_word =
            word_to_be_guessed.contains(*&user_input_letter.chars().next().unwrap());

        if is_letter_in_word == false {
            maximum_player_lives -= 1;
            continue;
        }

        let index_of_letter_in_word =
            word_to_be_guessed.match_indices(user_input_letter.chars().next().unwrap());

        for val in index_of_letter_in_word {
            let _value = std::mem::replace(
                &mut underscore_word[val.0],
                user_input_letter.chars().next().unwrap().to_string(),
            );
        }
    }

    if maximum_player_lives == 0 {
        println!("YOU ARE HANGED!");
        return;
    }

    println!("YOU WON!!!");
}
