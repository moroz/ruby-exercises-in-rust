// 题目: 使用者输入 x,y,z，请根据以下的判断输出结果
// 当 x < 0 输出 "A"
// 当 x > 0，且
//   当 y > 0，且
//     当 z > 0 输出 "B"
//     当 z < 0 输出 "C"
//   当 y < 0
//     当 z > 0 输出 "D"
//     当 z < 0 输出 "E"

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

fn abcde(x: isize, y: isize, z: isize) -> &'static str {
    if x == 0 || y == 0 || z == 0 {
        return "";
    }
    if x < 0 {
        return "A";
    }
    if y > 0 {
        if z > 0 {
            return "B";
        } else {
            return "C";
        }
    } else {
        if z > 0 {
            return "D";
        } else {
            return "E";
        }
    }
}

fn main() {
    println!("请输入一个整数x，然后按 Enter: ");
    let x = read_int();
    println!("请输入一个整数y，然后按 Enter: ");
    let y = read_int();
    println!("请输入一个整数z，然后按 Enter: ");
    let z = read_int();
    println!("结果是{}", abcde(x, y, z));
}
