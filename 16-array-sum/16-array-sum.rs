// 给定一阵列内含数字，输出最大值

fn find_max(arr: &Vec<i32>) -> &i32 {
    arr.iter().max().unwrap()
}

fn main() {
    let arr: Vec<i32> = vec![8, 12, 36, 53, 9, 75, 3, 71, 59, 88];

    let max = find_max(&arr);
    println!("Max is {max}"); // 应该是 88
}
