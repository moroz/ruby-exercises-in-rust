// 承上题，请排序并去除重复的数字
// Hint: 可用 arr.sort 排序，和 arr.uniq 去除重复

fn filter_even(arr: &Vec<i32>) -> Vec<&i32> {
    arr.iter().filter(|n| *n % 2 == 0).collect()
}

fn main() {
    let arr: Vec<i32> = vec![7, 68, 42, 46, 9, 91, 77, 46, 86, 1];
    let filtered = filter_even(&arr);
    let mut sorted = filtered.clone();
    sorted.sort();
    sorted.dedup();

    println!("{:?}", filtered);
    println!("{:?}", sorted);
    // puts "________" # 应该是 [42, 46, 68, 86]
    println!("Original array: {:?}", arr);
    // puts filter_even(arr).to_s # 应该是 [68, 42, 46, 46, 86]
}
