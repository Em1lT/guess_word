use text_io::read;
use rand::Rng;

fn main() {
    let list: [&str; 3] = ["dog", "cat", "cow"];
    let mut random_number = rand::thread_rng();
    let random_word = list[random_number.gen_range(0..list.len() - 1)];
    let tries: u32 = 3;

    println!("Guess a 3 letter word, 3 tries");
    println!("? ? ?");

    let word: String = read!();
    let corr: bool = random_word.eq(word.as_str());
    let nextGuess: String;

    if !word.is_empty() {
        for (i, c) in word.chars().enumerate() {
            let wordStr: &str = random_word.collect();
            println!("{}", c);
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
