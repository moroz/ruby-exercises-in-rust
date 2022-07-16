// 题目: 使用者输入 x,y,z，请输出三个数中最大的数

use std::io;

fn read_float() -> isize {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim()
        .parse()
        .expect("Failed to parse input as integer")
}

fn find_biggest_var_name(x: isize, y: isize, z: isize) -> &'static str {
    if x >= y && x >= z {
        return "x";
    }
    if y >= x && y >= z {
        return "y";
    }
    return "z";
}

fn main() {
    println!("请输入一个数字x，然后按 Enter: ");
    let x = read_float();

    println!("请输入一个数字y，然后按 Enter: ");
    let y = read_float();

    println!("请输入一个数字z，然后按 Enter: ");
    let z = read_float();

    let max = find_biggest_var_name(x, y, z);

    println!("最大的数是 {}", max);
}
