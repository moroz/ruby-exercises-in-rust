use std::io::{self, BufRead};

fn main() {
    println!("Please input your name, and hit Enter: ");

    let stdio = io::stdin();
    let name = stdio.lock().lines().next().unwrap().unwrap();

    println!("Hello, {}.", name);
}
