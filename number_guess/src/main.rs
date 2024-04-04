use std::io::stdin;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("guess out of range");
                        } else if guess > number {
                            println!("Guess is too higher");
                        } else if guess < number {
                            println!("Guess is too lower");
                        } else {
                            println!("correct");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read entry. {}", e)
                    }
                }
            },
            Err(_) => continue,
        }
    }
}
