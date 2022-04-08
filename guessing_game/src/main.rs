use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("I bet you can't guess my number ;P");
    
    let random_number = rand::thread_rng()
        .gen_range(1,101);
    
    println!("Go ahead, guess. I dare you");
    
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, goober. Try again!");
                continue;
            },
        };

        println!("You say: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Pfft, think bigger."),
            Ordering::Greater => println!("Woah, too big my dude!"),
            Ordering::Equal => {
                    println!("YOOOO!!!!! Nice! You nailed it!");
                    break;
            }
        }
    }
}
