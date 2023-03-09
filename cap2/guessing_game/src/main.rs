//fn main() {
//    println!("Hello, world!");
//}
use std::io::stdin;
//use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    //io::stdin()
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}