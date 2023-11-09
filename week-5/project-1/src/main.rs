use std::io;

fn main() {
    println!("Enter the coefficients of the quadratic equation:");

    // Input the values of a, b, and c
    let a: f64;
    let b: f64;
    let c: f64;

    let mut input = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    a = input.trim().parse().expect("Input not a valid number");
    input.clear();

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    b = input.trim().parse().expect("Input not a valid number");
    input.clear();

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    c = input.trim().parse().expect("Input not a valid number");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        println!("Two distinct real roots");
    } else if discriminant == 0.0 {
        println!("Exactly one real root");
    } else {
        println!("No real roots, discriminant is negative.");
    }
}
