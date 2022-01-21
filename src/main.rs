use text_io::read;

fn main() {
    println!("Guess a 3 letter word");

    let list: [&str; 3] = ["dog", "cat", "cow"];
    let word: String = read!();
    let corr: &str = if list.contains(&word.as_str()) { "correct" } else { "false" };

    println!("you got: {}", corr);
}
