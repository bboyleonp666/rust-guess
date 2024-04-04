use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the correct number");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_num}");

    loop {
        println!("Please input your guess ...");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Read failed ...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                let tmp_guess = guess.trim();
                println!("'{tmp_guess}' is not a valid number");
                continue;
            }
        };
        println!("Your guess is ... {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Try guess greater"),
            Ordering::Greater => println!("Try guess less"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
