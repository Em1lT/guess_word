use text_io::read;
use rand::Rng;
// use std::io::{self, Write};

fn ask_input()-> String {
    read!()
}

fn enumarate_answer(guess_word: String, correct_word: &str)-> String {
    let mut next_guess: String = "".to_owned();
    let word_str: Vec<char> = correct_word.chars().collect();

    for (i, c) in guess_word.chars().enumerate() {
        if c == word_str[i] {
            next_guess.push_str("O ")
        } else if correct_word.contains(c) {
            next_guess.push_str("! ")
        } else {
            next_guess.push_str("X ")
        }
    };

    next_guess
}

fn main() {
    let list: [&str; 3] = ["dog", "cat", "cow"];
    let mut random_number = rand::thread_rng();
    let random_word = list[random_number.gen_range(0..list.len() - 1)];

    println!(" [ Guess a 3 letter word, 3 tries ]");
    println!(" [ ? ? ? ]");
    let mut guess_correct: bool = false;
    let mut tries: u8 = 0;
    let total_tries: u8 = 3;

    while !guess_correct && tries != total_tries {
        let mut word: String = "".to_owned();

        while word.len() != 3 {
            let input: &String = &ask_input();
            if input.len() == 3 {
                word.push_str(input)
            } else {
                println!("Only 3 letters");
            }
        }

        if word == random_word {
            guess_correct = true;
        }
        let answer_row: String = enumarate_answer(word, random_word);

        println!("[ {}]", answer_row);
        tries = tries + 1;
    }
    let msg = if guess_correct { "You won!" } else { "You lost!" };
    println!("[ {} ]", msg);
}
