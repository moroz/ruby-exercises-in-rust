use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 题目 猜数字游戏：程序会先产生随机数，然后用户开始猜数字。程序会针对猜的数字回答太高、太低或猜中，猜中后程序就会终止。

fn read_int() -> u32 {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim()
        .parse()
        .expect("Failed to parse input as positive integer")
}

fn main() {
    let target = rand::thread_rng().gen_range(1..=100);
    println!("Target: {}", target);

    loop {
        let guess = read_int();

        match guess.cmp(&target) {
            Ordering::Less => {
                println!("太低了，再猜一次");
            }
            Ordering::Greater => {
                println!("太高了，再猜一次");
            }
            _ => {
                println!("恭喜猜中啦! ");
                break;
            }
        }
    }
}
