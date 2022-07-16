//  题目: 输入体重和身高，输出身体质量指数(BMI)和建议
//  BMI 公式为 bmi = ( 体重 / (身高x身高) )，单位是公斤和米
//  如果 BMI < 18.5，显示过轻
//  如果 BMI >= 24，显示过重
//  如果 BMI 介于 18.5 ~ 24，显示正常

use std::io;

fn read_float() -> f64 {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim().parse().expect("Failed to parse input as float")
}

fn main() {
    print!("请输入您的体重(公斤)，然后按 Enter: ");
    let weight = read_float();
    print!("请输入您的身高(厘米)，然后按 Enter: ");
    let height = read_float() / 100.0;

    if height == 0.0 {
        println!("Cannot divide by 0!");
        std::process::exit(1);
    }

    let bmi = weight / (height * height);
    println!("您的 BMI 是: {}", bmi);

    let result = match bmi {
        light if (0.0..18.5).contains(&light) => "过轻",
        normal if (18.5..=24.0).contains(&normal) => "正常",
        _ => "过重",
    };

    println!("您的 BMI 结果是: {}", result);
}
