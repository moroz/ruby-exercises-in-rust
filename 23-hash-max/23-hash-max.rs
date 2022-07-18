// 给定一 Hash，输出有最大 value 的 key

use std::collections::HashMap;

fn find_max<T>(hash: &HashMap<T, i32>) -> &T {
    let mut iter = hash.iter();
    let (mut max_key, mut max_val) = iter.next().unwrap();

    for (key, val) in iter {
        if val > max_val {
            max_val = val;
            max_key = key;
        }
    }

    return max_key;
}

fn main() {
    let hash = HashMap::from([("a", 71), ("b", 38), ("c", 21), ("d", 80), ("e", 10)]);
    let answer = find_max(&hash);
    println!("有最大 value 的是 {answer}");
}
