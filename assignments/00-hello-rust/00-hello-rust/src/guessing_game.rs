#![warn(missing_docs)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

///A program which runs a simple command line based number guessing game

///Prompts the command line and parses user input for valid integers.
///Returns:
/// 32 bit integer read from user string input from command line
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new(); //create an empty string for the input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); //read input from command line into string

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    } //parse string input and return as integer if valid
}
///Runs the random number guessing game. Prompts for user input from command line to facilitate guess and comparison.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random integer in range 1 to 100

    loop {
        let guess = get_input(); //get user's guess from command line
        print!("You guessed: {}. ", guess);

        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        } //check whether the users guess is too high, too low, or correct
    }
}
