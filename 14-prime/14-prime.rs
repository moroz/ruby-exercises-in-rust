// # 输入一个数字 N，请检查是不是质数

use std::io;

fn read_int() -> u32 {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim()
        .parse()
        .expect("Failed to parse input as positive integer")
}

fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    let root = (n as f64).sqrt() as u32;
    for i in 2..=root {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("请输入数字 N，然后按 Enter: ");
    let n = read_int();

    if is_prime(n) {
        println!("这是质数");
    } else {
        println!("这不是质数");
    }
}
