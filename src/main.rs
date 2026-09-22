use std::{fs, io};

fn main() {
    let mut path = String::new();

    if io::stdin().read_line(&mut path).is_err() {
        println!("failure");
        return;
    }

    let path = path.trim();

    if path.is_empty() {
        println!("failure");
        return;
    }

    match fs::read(path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
