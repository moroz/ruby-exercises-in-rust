// 题目: 输入有多少片比萨饼和多少人，输出每人可以分到几片，以及剩下几片

use std::io;

fn read_int() -> usize {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim()
        .parse()
        .expect("Failed to parse input as unsigned integer")
}

fn main() -> io::Result<()> {
    println!("请输入有多少片比萨饼，然后按 Enter: ");
    let pizzas = read_int();

    println!("请输入有多少人要吃，然后按 Enter: ");
    let people = read_int();

    if people == 0 {
        println!("Cannot divide by 0!");
        std::process::exit(1);
    }

    println!("每人可分得几片: {} 片", pizzas / people);
    println!("还剩下几片: {} 片", pizzas % people);

    Ok(())
}
