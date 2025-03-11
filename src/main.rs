use std::cmp::Ordering;

use std::io;
use rand::Rng;
fn main() {
    println!("Guess a Number!");

    let secret_number = rand::thread_rng().gen_range(1..=50);
    loop {

    println!("Please input your guess.");

    let mut guess = String::new();

     io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
        
    println!("you guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }

}
}
