//Guess the number game

use rand::*;
use colour::*;

//function to get a random number between 1 and 100
fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..101)
}

//function to get the user input
fn get_user_input() -> u32 {
    let mut input = String::new();
    println!("\n-Please enter a number between 1 and 100-");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    input.trim().parse().expect("Not a valid number")
}

fn main(){
    white!("Welcome to the Guess the Number Game!\n");
    white!("I'm thinking of a number between 1 and 100\n");
    white!("Try to guess it");

    let mut guess = get_user_input();
    let random_number = get_random_number();
    let mut tries = 1;

    while guess != random_number {
        if guess > random_number {
            red!("Your guess is too high");
        } else {
            blue!("Your guess is too low");
        }
        guess = get_user_input();
        tries += 1;
    }
    
    green!("You guessed it! The number was {}\n", random_number);
    green!("It took you {} tries to guess the number!\n", tries);
}