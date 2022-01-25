use text_io::read;
use rand::Rng;

fn ask_input()-> String {
    let word: String = read!();
    word
}

fn main() {
    let list: [&str; 3] = ["dog", "cat", "cow"];
    let mut random_number = rand::thread_rng();
    let random_word = list[random_number.gen_range(0..list.len() - 1)];

    println!("Guess a 3 letter word, 3 tries");
    println!("? ? ?");

     let word: String = ask_input();
     let mut next_guess: String = "".to_owned();


    if !word.is_empty() {
        for (i, c) in word.chars().enumerate() {
            let word_str: Vec<char> = random_word.chars().collect();

            if c == word_str[i] {
                next_guess.push_str("O ")
            } else if random_word.contains(c) {
                next_guess.push_str("! ")
            } else {
                next_guess.push_str("X ")
            }
        };
        println!("{}", next_guess);
    } else {
        println!("No input");
    };
}
