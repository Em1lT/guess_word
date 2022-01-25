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

    let nextGuess: String;

    if !word.is_empty() {
        for (i, c) in word.chars().enumerate() {
            let wordStr: Vec<char> = random_word.chars().collect();
            println!("{}", c);
            println!("{}", wordStr[i]);

            // if c == wordStr {
            //     nextGuess += "O "
            // } else if random_word.chars().contains(c) {
            //     nextGuess += "O "
            // };
        };
    } else {
        println!("No input");
    };
}
