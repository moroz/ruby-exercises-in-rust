// 题目: 输入一个数字 x，请判断是否正数、零或负数，以及是不是偶数

use std::io;

fn read_int() -> isize {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim()
        .parse()
        .expect("Failed to parse input as integer")
}

fn main() {
    println!("请输入一个整数，然后按 Enter: ");
    let x = read_int();

    let sign = match x {
        neg if neg < 0 => "负数",
        0 => "零",
        _ => "正数",
    };

    println!("这个数是{}", sign);

    let even = if x % 2 == 0 { "偶数" } else { "奇数" };
    println!("这个数是{}", even);
}
