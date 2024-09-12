use std::io;
use rand::Rng;
use std::cmp::Ordering;

extern crate rand;

// Function to handle user input and compare guesses
fn input(secret_num: i32) -> (i32, i32) {
    println!("Hey there, are you ready to give the input ...!");

    loop {
        let mut guess = String::new(); // Prepare to read user input
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert input to integer and handle invalid input
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess);

        // Check if the guess matches the secret number
        if game(guess, secret_num) {
            return (guess, secret_num); // Return when correct guess is made
        }
    }
}

// Function to compare the guess with the secret number
fn game(guess: i32, secret_num: i32) -> bool {
    match guess.cmp(&secret_num) {
        Ordering::Less => {
            println!("Too small");
            false
        }
        Ordering::Greater => {
            println!("Too big");
            false
        }
        Ordering::Equal => {
            println!("Hurray! You got it!");
            true
        }
    }
}

fn main() {
    let secret_num = rand::thread_rng().gen_range(0..=100);
    input(secret_num);
}
