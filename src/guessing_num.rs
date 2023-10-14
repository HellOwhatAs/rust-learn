use std::io::Write;
use rand::Rng;

#[allow(dead_code)]
pub fn main() {
    println!("Guess a number (1 <= number <= 100)");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut input = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_owned();
        print!("You guessed {}, which is ", input);
        match secret_number.cmp(&input.parse().unwrap()) {
            std::cmp::Ordering::Less => println!("too large"),
            std::cmp::Ordering::Greater => println!("too small"),
            std::cmp::Ordering::Equal => {
                println!("correct!");
                break
            }, 
        }
        input.clear();
    }
}