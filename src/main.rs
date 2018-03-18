extern crate rand;

use std::io;
use rand::Rng;

fn main() {

    println!("Guess something");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("secret number:{}", secret_number);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed");
    
    println!("Guess: {}", guess);
}
