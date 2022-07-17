// 给定一阵列内含数字，输出另一个数组只包含偶数

fn filter_even(arr: &Vec<i32>) -> Vec<&i32> {
    arr.iter().filter(|n| *n % 2 == 0).collect()
}

fn main() {
    let arr: Vec<i32> = vec![7, 68, 42, 46, 9, 91, 77, 46, 86, 1];
    let filtered = filter_even(&arr);

    println!("{:?}", filtered);
    println!("Original array: {:?}", arr);
    // puts filter_even(arr).to_s # 应该是 [68, 42, 46, 46, 86]
}
