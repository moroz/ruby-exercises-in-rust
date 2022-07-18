// 计算一个阵列中各个元素的出现频率

use std::collections::HashMap;

fn count<'a>(arr: &Vec<&'a str>) -> HashMap<&'a str, i32> {
    let mut counts = HashMap::new();

    for key in arr {
        if let Some(count) = counts.get_mut(*key) {
            *count += 1;
        } else {
            counts.insert(*key, 1);
        }
    }

    counts
}

fn main() {
    let arr = vec![
        "a", "d", "d", "c", "b", "c", "c", "c", "d", "d", "e", "e", "e", "d", "a", "c", "e", "a",
        "d", "e",
    ];
    let answer = count(&arr);
    println!("{:#?}", answer); // 答案应该是 {"a"=>3, "d"=>6, "c"=>5, "b"=>1, "e"=>5}
}
