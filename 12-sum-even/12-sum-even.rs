// 题目: 求 1~100 所有偶数的和

fn main() {
    let mut i = 1;
    let mut total = 0;

    while i <= 100 {
        if i % 2 == 0 {
            total += i;
        }
        i += 1
    }
    println!("{}", total);
}

// 不智障的算法：
// https://www.matemaks.pl/suma-ciagu-arytmetycznego.html
// f(n) = 2n
// sum = (f(1) + f(50)) / 2 * 50
// sum = (2 + 100) / 2 * 50
// sum = 51 * 50
// sum = 2550
