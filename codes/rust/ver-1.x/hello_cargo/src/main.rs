extern crate rand;
extern crate adder;
extern crate boxs_rc;
extern crate lifetimes;
extern crate parallel_helper;
extern crate patterns;
extern crate webserver;
extern crate advances;
extern crate methods_lib as methodsLib;

use std::env;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod libs;

//main code

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // add code here
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn log(&self) {
        println!("{:?}", self);
    }
}

impl Rectangle {
    // add code here
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


fn run_with_subs() {
    let num  = 10;
    println!("Hi, I'm {:?} years old ...", adder::add_one(num));

    //boxs_rc::run();
    //parallel_helper::run();
    //patterns::run();
    //webserver::run();
    //advances::run();
    methodsLib::run();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("See some args = {:?}", args);

    run_with_subs();

    return;

    // ---
    test_string();

    // call others modules
    libs::netwrok::client::workout();
    libs::netwrok::client::connect();
    //libs::error::load_file();
    libs::error::read_username_from_file();



    let s = String::from("hello world");
    let word = first_word(&s);
    println!("====>: {}", word);
    
    let rect1 = Rectangle { width: 20, height: 100};
    println!("{:?}", rect1.area());
    rect1.log();

    let square1 = Rectangle::square(40);
    square1.log();

    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    println!("six = {:?}", six);

    
}

fn test_string() {
    let s1 = String::from("Hello,");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    println!("{:?}", s3);
    println!("{:?}", &s1);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
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
