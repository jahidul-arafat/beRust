use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Generating a Secret Number between [1 to 100]...");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The Secret Number is: {}",secret_number);

    println!("Guess the number [1 to 100]");
    loop {
        println!("Please input your guess [1 to 100].");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You Guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
