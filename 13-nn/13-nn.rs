// 题目: 输入一个数字 N，输出 N * N 乘法表

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

fn main() {
    println!("请输入数字 N，然后按 Enter: ");
    let n = read_int();

    for i in 1..=n {
        for j in 1..=n {
            let res: u64 = (i as u64) * (j as u64);
            println!("{} * {} = {}", i, j, res);
        }
    }
}
