use std::io;

fn read_float() -> f64 {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim().parse().expect("Failed to parse input as float")
}

// 题目: 输入直角三角形的宽和高，输出三角形的面积

fn calculate_area(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

fn main() {
    println!("请输入直角三角形的高，然后按 Enter: ");
    let a = read_float();

    println!("请输入直角三角形的底边，然后按 Enter: ");
    let b = read_float();

    let answer = calculate_area(a, b);

    println!("直角三角形的面积是: {}", answer);
}
