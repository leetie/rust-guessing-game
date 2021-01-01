use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let range_choice = get_range_choice();
    let secret_number = rand::thread_rng().gen_range(1, range_choice);
    println!("Please input your guess.");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You guessed: {}", guess);
    }
}
fn get_range_choice() -> u32 {
    println!("Choose a number as an upper bound for guessing range. (ie 200 for 1-200)");
    // linter complains that the value stored in the variable below is never read, even though the return after the loop is in the same scope?? ..investigate
    let mut result = 0;
    loop {
        let mut range_choice = String::new();
        io::stdin()
            .read_line(&mut range_choice)
            .expect("Failed to read line");
        let range_choice: u32 = match range_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        result = range_choice;
        break;
    }
    return result;
}
