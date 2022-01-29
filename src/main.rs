use text_io::read;
use rand::Rng;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::borrow::Borrow;

// use std::io::{self, Write};

fn ask_input()-> String {
    read!()
}

fn enumarate_answer(guess_word: String, correct_word: String)-> String {
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

fn answer()-> String {
    let mut word: String = "".to_owned();
    while word.len() != 5 {
        let input: &String = &ask_input();
        if input.len() == 5 {
            word.push_str(input);
        } else {
            println!("Only 5 letters");
        }
    }
    word
}
fn start_game(winning_word: String) {
    let mut guess_correct: bool = false;
    let mut tries: u8 = 0;
    let total_tries: u8 = 3;

    while !guess_correct && tries != total_tries {
        let user_answer = answer();
        if user_answer == winning_word {
            guess_correct = true;
        }
        let answer_row: String = enumarate_answer(user_answer, winning_word);
        println!("[ {}]", answer_row);
        tries = tries + 1;
    }
    let msg = if guess_correct { "You won!" } else { "You lost!" };
    println!("[ {} ]", msg);
}

fn random_word()-> String {
    // let list: [&str; 3] = ["dogas", "catsr", "cowll"];
    let mut random_number = rand::thread_rng();
    let mut random_word: String = "".to_string();
    let mut cnt: u8 = 0;
    let path = Path::new("./lists/words.txt");
    let file = BufReader::new(File::open(&path).expect("Unable to open file")).lines();
    let file_length = file.borrow().clone().count();
    let random_index = random_number.gen_range(0..file_length);

    // let file_lines = file;
    for line in file {
        if usize::from(cnt) == random_index {
            println!(" [ {:?} ]", &line.unwrap());
            break;
        }
        cnt = cnt + 1;
    }

    random_word.push_str("horse");
    random_word
}

fn setup() {
    let random_word = random_word();
    println!(" [ Guess a 5 letter word, 3 tries ]");
    println!(" [ ? ? ? ? ? ]");
    start_game(random_word);
}

fn main() {
    // setup();
    let paska = random_word();
    println!(" [ {} ]", paska);
}
