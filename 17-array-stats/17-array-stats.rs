// 使用者不断输入数字存进 Array，最后输出总和、平均、最大值、最小值

use std::io;

fn read_number() -> Option<isize> {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim().parse().ok()
}

fn main() {
    let mut arr: Vec<isize> = Vec::new();
    loop {
        println!("请输入数字，结束请直接按 Enter: ");
        let user_input = read_number();
        if let Some(user_input) = user_input {
            arr.push(user_input);
        } else {
            break;
        }
    }

    println!("{:?}", arr);

    let sum: &isize = &arr.iter().sum();
    let len = &arr.iter().len();
    println!("总和是 {sum}");
    let avg = (*sum as f64) / (*len as f64);
    println!("平均是 {avg}");
    let min = &arr.iter().min().unwrap();
    let max = &arr.iter().max().unwrap();
    println!("最大值是 {min}");
    println!("最小值是 {max}");
}
