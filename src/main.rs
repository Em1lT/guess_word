use text_io::read;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

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
fn start_game(winning_word: String, total_tries: u8) {
    let mut guess_correct: bool = false;
    let mut tries: u8 = 0;

    while !guess_correct && tries != total_tries {
        let user_answer = answer();
        if user_answer == winning_word {
            guess_correct = true;
        }
        let answer_row: String = enumarate_answer(user_answer, winning_word.to_string());
        println!("[ {}]", answer_row);
        tries = tries + 1;
    }
    let msg = if guess_correct { "You won!" } else { "You lost!" };
    println!("[ {} ] \n[   {}   ]", msg, winning_word);
}

fn random_word()-> String {
    let word_list = read_file();
    let random_number: u16 = rand::thread_rng().gen_range(0..500);
    let mut random_word: String = "".to_string();
    let mut cnt: u16 = 0;
    for line in word_list {
        if cnt == random_number {
            // println!("{}", &line.unwrap());
            random_word.push_str(&line.unwrap());
        }
        cnt = cnt + 1;
    }
    random_word
}

fn read_file () -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let path = Path::new("./lists/words.txt");
    let file = BufReader::new(File::open(&path).expect("Unable to open file"));
    file.lines()
}

fn setup() {
    let random_word = random_word();
    let total_tries: u8 = 5;
    println!(" [ Guess a 5 letter word, {} tries ]", total_tries);
    println!(" [ ? ? ? ? ? ]");
    start_game(random_word, total_tries);
}

fn main() {
    setup();
}
