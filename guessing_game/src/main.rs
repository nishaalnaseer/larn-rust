use std::{cmp::Ordering, io};
use rand::Rng; // 0.8.5

fn guessing_game() {
    println!("Hello, welcom to guessing game!");
    let _num = rand::thread_rng().gen_range(0..=100);

    println!("A random number has been selected.");
    println!("The secret number is: {}", _num);

    loop {
        println!("Enter a number between 1 and 100 to make your guess!");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line"
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!("You guessed {}", guess);

        match guess.cmp(&_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}


fn test_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[5..12];
    println!("{hello}");
    println!("{world}");    
}
