use std::io;

fn read_number() -> Option<isize> {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line from STDIN!");
    buf.trim().parse().ok()
}

fn main() {
    println!("Please enter integers, one per line. The program will stop when you enter a non-integer (or empty line).");

    let mut arr = Vec::<isize>::new();

    while let Some(number) = read_number() {
        arr.push(number);
    }

    if arr.iter().len() == 0 {
        println!("No numbers were found!");
        std::process::exit(1);
    }

    let max = arr.iter().max().unwrap();

    println!("The biggest number is {}", max);
}
