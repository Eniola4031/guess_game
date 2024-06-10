use std::io;
use rand::Rng;
use std::cmp::Ordering;



//This is the first part of the guessing game
//Code that gets a guess from the user and prints it

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
  //  println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

    let mut guess = String::new();

//input and error handling
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
             break;

        }
             

        }
    }
}