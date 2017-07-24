extern crate rand;

use std::io;
use rand::Rng;


fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read from standard input");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("You guessed: {}", guess);

}
