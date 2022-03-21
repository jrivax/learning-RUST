use std::io;
use core::str::Chars;
use std::borrow::Borrow;

fn main() {
    println!("Guess the word!");
    
    let guess_word: String = String::from("hangman");
    let letters_used: String = String::new();
    // let guess_word_vec = guess_word.chars();
    let mut letter = String::new();
    let mut attempts: i8 = 5;
    
    loop {
      let mut letters_used_clone = letters_used.clone();
      println!("Please input a letter to guess: ");
      io::stdin()
        .read_line(&mut letter)
        .expect("Failed to read letter");
  
      // if guess_word.contains(&letter.to_string()) {
      //   println!("attemps")
      // }
      letters_used_clone += &letter;
      println!("{}", letters_used_clone);
      // let win = print_guess_word(letters_used_clone, &guess_word_vec);
      print_guess_word(letters_used_clone, &guess_word);
      // if win {
      //   println!("You win!");
      //   break;
      // } else {
      //   // if !successful_attempt { attempts -= 1 };
      //   if attempts == 0 {
      //     println!("You loose!");
      //     break;
      //   }else{
      //     println!("You have {} attempts left", attempts);
      //   }
      // }
    }
}
// FUNCTION
fn print_guess_word(letters_used: String, guess_word: &String) {
  let mut print_string: Vec<String> = Vec::new();
  // let guess_word: String = String::from("hangman");
  let guess_word_vec = guess_word.chars();

  guess_word_vec.for_each(|letter| {
    if letters_used.contains(letter) {     
      print_string.push(letter.to_string());
    }else {   
      print_string.push("_".to_string());
    }
  });
  let printable_word: String = print_string.join("  ");
  println!("{}", printable_word);
}
