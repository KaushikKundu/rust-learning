use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("guess the number");

    let secret = rand::thread_rng().gen_range(1..=100);
   
    loop {
        println!("input your guess");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
        // readline returns result which is an enumeration of two variants. ok and err. to handle err, expect is added.
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess: {}",guess);

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You guessed the number correctly!");
                break;
            } 
        }
    }
    
}
