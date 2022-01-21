use text_io::read;

fn main() {
    let list: [&str; 3] = ["dog", "cat", "cow"];
    println!("Guess a 3 letter word");
    let word: String = read!();
    let corr: &str = if list.contains(&word as &str) { "correct" } else { "false" };
    println!("you got: {}", corr);
}
