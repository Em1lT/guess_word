use text_io::read;
use rand::Rng;

fn ask_input()-> String {
    read!()
}

fn main() {
    let list: [&str; 3] = ["dog", "cat", "cow"];
    let mut random_number = rand::thread_rng();
    let random_word = list[random_number.gen_range(0..list.len() - 1)];

    println!(" [ Guess a 3 letter word, 3 tries ]");
    println!(" [ ? ? ? ]");
    let mut next_guess: String = "".to_owned();
    let mut guess_correct: bool = false;
    let mut tries: u8 = 0;
    let total_tries: u8 = 3;

    while !guess_correct && tries != total_tries {
        next_guess.clear();
        let mut word: String = "".to_owned();
        //while word.len() != 3 {
            word.push_str(&ask_input());
        // }

        for (i, c) in word.chars().enumerate() {
            let word_str: Vec<char> = random_word.chars().collect();

            if word == random_word {
                guess_correct = true;
                break
            }

            if c == word_str[i] {
                next_guess.push_str("O ")
            } else if random_word.contains(c) {
                next_guess.push_str("! ")
            } else {
                next_guess.push_str("X ")
            }
        };
        println!("[ {}]", next_guess);
        tries = tries + 1;
    }
}
