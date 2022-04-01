use std::io;
/**
 * Summary: Hangman game.
 * Description: Print screen status.
 */
fn main() {
    println!("Welcome to the hangman game");
    println!("let's see if you can guess the word in less than 5 tries!");
    
    let guess_word: String = String::from("hangman");
    let letters_used: String = String::new();
    let mut input_letter = String::new();
    let mut attempts: i8 = 5;
    
    loop {
      let mut letters_used_clone = letters_used.clone();
      println!("Please enter a letter to guess: ");
      io::stdin()
        .read_line(&mut input_letter)
        .expect("Failed to read letter");
  
      letters_used_clone += &input_letter;
      println!("{}", letters_used_clone);

      print_guess_word(&letters_used_clone, &guess_word);
      if check_win(&letters_used_clone, &guess_word){
        println!("YOU WIN !!!!");
        break;
      }
      attempts -= 1;
      if attempts == 0{
        println!("YOU LOOSE :( ");          
        break;
      }

    }
}
/**
 * Summary: Print screen status..
 * Description: Print screen status.
 * * @param {letters_used: &String}
 * * @param {guess_word: &String}
 * * @return ()
 */
fn print_guess_word(letters_used: &String, guess_word: &String) {
  let mut print_string: Vec<String> = Vec::new();

  guess_word.chars().for_each(|letter| {
    if letters_used.contains(letter) {     
      print_string.push(letter.to_string());
    }else {   
      print_string.push("_".to_string());
    }
  });
  let printable_word: String = print_string.join("  ");
  println!("{}", printable_word);
}
/**
 * Summary: Check if user wins the game.
 * Description: Check if user wins the game.
 * * @param {letters_used: &String}
 * * @param {guess_word: &String}
 * * @return {bool}
 */
fn check_win(letters_used: &String, guess_word: &String) -> bool {
  
  let mut win = true;
  guess_word.chars().for_each(|letter| {
    if !letters_used.contains(letter) {
      win = false;
    }
  });
  return win;
}  
