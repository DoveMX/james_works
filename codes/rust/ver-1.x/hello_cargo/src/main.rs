extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
   
    show_hello();
    
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("====>: {}", word);
    
    
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn show_hello() {
    println!("Hello, world!");
}

fn first_word(s: &str) -> &str {
   let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


#[warn(igonedead_code)]
fn test_ownership(some_string: &mut String) {
    println!("(1) {}, world!", some_string);
    some_string.push_str(", world!");
}

#[warn(igonedead_code)]
fn guess_things() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is :{0}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
