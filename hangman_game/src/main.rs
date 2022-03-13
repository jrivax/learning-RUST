use std::io;

fn main() {
    println!("Guess the word!");
    
    let guess_word = "hangman";
    let letters_used: String = String::new();
    let guess_word_vec: Vec<char> = guess_word.chars().collect();
    let mut letter = String::new();

    loop {
      let mut letters_used_clone = letters_used.clone();
      println!("Please input a letter to guess: ");
      io::stdin()
        .read_line(&mut letter)
        .expect("Failed to read letter");
  
      letters_used_clone += &letter;
      println!("{}", copy);
  
      let win = print_guess_word(copy, &guess_word_vec);
      if win {
        println!("You win!");
        break;
      }
    }
}
// FUNCTION
fn print_guess_word(letters_used: String, guess_word_vec: &Vec<char>) -> bool  {
  let mut print_string: Vec<String> = Vec::new();
  let mut flag_win: bool = true;

  if letters_used.len() == 0 {
    for _letter in guess_word_vec {
      print_string.push("_".to_string());
    }
  } else {
    for _letter in guess_word_vec {
      let mut _letter_copy;
      _letter_copy = _letter.to_string();
      if letters_used.contains(&_letter_copy) {
        print_string.push(_letter.to_string());
      }else {   
        flag_win = false;     
        print_string.push("_".to_string());
      }
    }
  }
  let s: String = print_string.join("  ");
  println!("{}", s);
  return flag_win;
}
