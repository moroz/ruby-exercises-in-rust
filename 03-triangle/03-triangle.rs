use std::io;

fn read_float() -> f64 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let f: f64 = buf.trim().parse().unwrap();
    f
}

fn main() -> io::Result<()> {
    println!("Please input the height of a right triangle:");
    let a = read_float();

    println!("Please input the width of a right triangle:");
    let b = read_float();

    let result = (a * a + b * b).sqrt();

    println!("The area of this square triangle is {}", result);

    Ok(())
}
