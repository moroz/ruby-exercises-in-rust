// 给定一数组内含数字，请实作选择排序法进行排序。
// https://zh.wikipedia.org/wiki/选择排序

fn selection_sort(arr: &Vec<i32>) -> Vec<i32> {
    let length = arr.len();
    let mut new_arr = arr.clone();

    for i in 0..length {
        let mut j_min = i;

        for j in (i + 1)..length {
            if new_arr[j] < new_arr[j_min] {
                j_min = j;
            }
        }

        if j_min != i {
            let buf = new_arr[i];
            new_arr[i] = new_arr[j_min];
            new_arr[j_min] = buf;
        }
    }

    new_arr.to_vec()
}

fn main() {
    let arr = vec![7, 68, 42, 46, 9, 91, 77, 46, 86, 1];

    let answer = selection_sort(&arr);
    println!("{:?}", answer);
    println!("Original array: {:?}", arr);
}
