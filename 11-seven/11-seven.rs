// 题目: 列出 1 到 100 之间，所有 7 的倍数

fn main() {
    let mut i = 1;

    while i <= 100 {
        if i % 7 == 0 {
            println!("{}", i);
        }
        i += 1;
    }
}