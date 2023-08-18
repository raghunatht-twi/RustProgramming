//implementing the guessing game program 

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //basics of print statement
    let x = 5;
    let y = 10;

    println!("x = {x}, y = {y}, x+y^2 = {}", x+y*y);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=1000);
    //println!("The secret number is: {secret_number}");

    println!("Please input your guess");

    loop {

	    let mut guess = String::new();

	    io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	    let guess: u32 = guess
		.trim()
		.parse()
		.expect("Please type a number!");

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

    println!("Hello, world! Checking build. Who is to be blamed for changes?");
}
