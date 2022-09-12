use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your Number!");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail to read!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("The secrect number is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too greater"),
            Ordering::Less => println!("Too less"),
        }

        println!("Your Number is {guess}");
    }
}
