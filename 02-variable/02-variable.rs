fn main() {
    let mut a = 1;
    let mut b = 2;

    println!("a is {}", a);
    println!("b is {}", b);

    let buffer = b;

    b = a;
    a = buffer;

    println!("a was 1, now it is {}", a);
    println!("b was 2, now it is {}", b);
}
