use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("woooo!");

    let mut rng = rand::thread_rng();

    let secret_number: u8 = rng.gen_range(0..=100);

    let mut counter: u32 = 0;

    println!("guess numebr");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("shit bro you cant write?");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("wtf bro its supposed to be a number");
                continue;
            }
        };

        counter += 1;

        println!("you guessd {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small :("),
            Ordering::Equal => {
                println!("you win in {} guesses :D", counter);
                break;
            },
            Ordering::Greater => println!("too much :(")
        }
    }
    
}
