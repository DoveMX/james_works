
use std::io;
use std::io::Read;
use std::fs::File;

pub fn load_file() {
    let use_unwrap = false;
    if use_unwrap {
        let f = File::open("hello.txt").unwrap();
    }

    let f = File::open("hello.txt").expect("---> Can't find hello.txt");
}

pub fn read_username_from_file() {
    _read_username_from_file().expect("---- Test");
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

