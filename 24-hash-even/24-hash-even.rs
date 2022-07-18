// 给定一 Hash，输出 value 是偶数的 keys

use std::collections::HashMap;

fn find_even_keys<T>(hash: &HashMap<T, i32>) -> Vec<&T> {
    hash.iter()
        .filter_map(|(k, v)| if v % 2 == 0 { Some(k) } else { None })
        .collect()
}

fn main() {
    let hash = HashMap::from([("a", 71), ("b", 38), ("c", 21), ("d", 80), ("e", 10)]);
    let mut answer = find_even_keys(&hash);
    answer.sort();
    let printed = &answer.iter().map(|k| **k).collect::<Vec<_>>().join(", ");
    println!("有偶数大 value 的 keys 有 [{}]", printed);
}
