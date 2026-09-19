use std::{fs, io};

fn main() {
    let mut path = String::new();

    if io::stdin().read_line(&mut path).is_err() {
        println!("failure");
        return;
    }

    if path.trim().is_empty() {
        print!("failure");
        return;
    }
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            print!("failure");
            return;
        }
    };

    print!("success");
}
