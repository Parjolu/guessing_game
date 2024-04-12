use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Ghiceste numarul");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Scrie numarul.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ai ghicit: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if guess <= secret_number / 2 {
                    println!("Mult prea mic!")
                } else {
                    println!("Prea mic bro")
                }
            }
            Ordering::Greater => {
                if guess >= secret_number * 3 / 2 {
                    println!("Mult prea mare!");
                } else {
                    println!("Prea mare bro")
                }
            }
            Ordering::Equal => {
                println!("Bravo ba!");
                break;
            }
        }
    }
}

