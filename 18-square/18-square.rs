// # 建构一个阵列有一百个的元素，内容是 0, 1, 4, 9, 16, 25...... 每个元素是该索引的平方

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

fn build_array(n: u32) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }
    // Ranges are iterable and therefore have no .iter() method
    (1..=n).map(|n| n * n).collect()
}

fn main() {
    println!("请输入数字 N，然后按 Enter: ");
    let n = read_int();
    let arr = build_array(n);
    println!("{:?}", arr);
}
