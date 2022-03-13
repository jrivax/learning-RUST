use std::io;

fn main() {
    println!("Guess the word!");
    println!("Please input a letter to start guessing.");
    let guess_word = "hangman";
    let mut letters_used: Vec<char> = Vec::new();
    let guess_word_vec: Vec<char> = guess_word.chars().collect();

    let mut letter = String::new();
    print_guess_word(letters_used, guess_word_vec);
    io::stdin()
      .read_line(&mut letter)
      .expect("Failed to read letter");

    println!("Your guessed letter: {}", letter);
}
fn print_guess_word(letters_used: Vec<char>, guess_word_vec: Vec<char>) {
  let mut print_string = String::new();
  let mut print_string = Vec::new();

  if(letters_used.len() == 0){
    for letter in guess_word_vec {
      //print_string = concat!("_ ").to_string();
      print_string.push("_");
    }
  }
  let s: String = print_string.connect("  ");

  println!("{}", s);
}
