use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first edge of the triangle: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the second edge of the triangle: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the third edge of the triangle: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let s = (a + b + c) / 2.0;
    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

    println!("Area of the triangle: {}", area);
}
