
use std::io;

fn main() {
    println!("Guess a Number!");

    println!("Please input your guess.");

     let mut guess = String::new();

     io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("you guessed: {}",guess);



}
