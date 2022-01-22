use text_io::read;
use rand::Rng;

fn main() {
    let list: [&str; 3] = ["dog", "cat", "cow"];
    let mut random_number = rand::thread_rng();
    let random_word = list[random_number.gen_range(0..list.len() - 1)];

    println!("Guess a 3 letter word");
    println!("? ? ?");
    let word: String = read!();

    println!("{}", random_word);
    let corr: &str = if random_word.eq(&word.as_str()) { "correct" } else { "false" };

    println!("you got: {}", corr);

}
