// 给定一阵列内含数字，请输出 0~9 中不见的数字

use std::collections::HashSet;

fn find_missing(arr: &Vec<i32>) -> Vec<i32> {
    let mut set: HashSet<_> = (0..=9).collect();
    for elem in arr {
        if set.contains(&elem) {
            set.remove(&elem);
        }
    }
    let mut vec: Vec<_> = set.drain().collect();
    vec.sort();
    vec
}

fn main() {
    let arr = vec![2, 2, 1, 5, 8, 4];
    let answer = find_missing(&arr);
    println!("{:?}", answer);
    // puts answer.to_s # 应该是 [0,3,6,7,9]
}
