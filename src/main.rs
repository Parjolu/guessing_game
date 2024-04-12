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
            Ordering::Less => println!("Prea mic!"),
            Ordering::Greater => println!("Prea mare!"),
            Ordering::Equal => {
                println!("Bravo ba!");
                break;
            }
        }
    }
}

