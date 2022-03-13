use std::io;

fn main() {
    println!("Guess the word!");
    println!("Please input a letter to start guessing.");
    let guess_word = "hangman";
    let mut letters_used: String = String::new();
    let guess_word_vec: Vec<char> = guess_word.chars().collect();

    let mut letter = String::new();
    // print_guess_word(letters_used, guess_word_vec);
    io::stdin()
      .read_line(&mut letter)
      .expect("Failed to read letter");

    letters_used += &letter;
    println!("{}", letters_used);

    print_guess_word(letters_used, guess_word_vec);
    // println!("Your guessed letter: {}", letter);
}
// FUNCTION
fn print_guess_word(letters_used: String, guess_word_vec: Vec<char>) {
  let mut print_string: Vec<String> = Vec::new();

  if letters_used.len() == 0 {
    for _letter in guess_word_vec {
      print_string.push("_".to_string());
    }
  } else {
    for _letter in guess_word_vec {
      if letters_used.contains(_letter) {
        print_string.push(_letter.to_string());
      }else {        
        print_string.push("_".to_string());
      }
    }
  }
  let s: String = print_string.join("  ");

  println!("{}", s);
}
