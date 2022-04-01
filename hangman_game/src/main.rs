use std::io;
use std::{
  fs::File,
  io::{BufRead, BufReader},
};
use rand::seq::IteratorRandom;
/**
 * Summary: Hangman game.
 * Description: Print screen status.
 */

const FILENAME: &str = "src/words_to_guess.txt";

fn main() {
    println!("Welcome to the hangman game");
    println!("let's see if you can guess the word in less than 5 tries!");

    let mut letters_used: String = String::new();
    let mut input_letter = String::new();
    let mut attempts: i8 = 5;
    let guess_word = get_random_word(FILENAME);
    loop {
      println!("Please enter a letter to guess: ");
      print!("{}[2J", 27 as char);
      io::stdin()
        .read_line(&mut input_letter)
        .expect("Failed to read letter");

      letters_used.push(input_letter.trim().replace("\n", "").chars().last().unwrap());
      println!("You guessed the following letters:{}", letters_used);

      print_guess_word(&letters_used, &guess_word);
      if check_win(&letters_used, &guess_word){
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

fn get_random_word(file_name: &str) -> String {

  let file = File::open(file_name).unwrap_or_else(|err| {
    panic!("Error opening file: {} : {}", FILENAME, err);
  });
  let file = BufReader::new(file);
  let lines = file.lines().map(|line| line.expect("Could not read line"));

  return lines
    .choose(&mut rand::thread_rng())
    .expect("File has no lines");
}
