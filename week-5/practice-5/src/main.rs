use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter Your Height (in centimetres):");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let height: f32 = input.trim().parse().expect("Input not a valid number");

    if height >= 150.0 && height <= 170.0 {
        println!("You are of average height");
    } else if height > 170.0 && height < 195.0 {
        println!("You are tall");
    } else if height < 150.0 && height > 100.0 {
        println!("You are a dwarf");
    } else {
        println!("Abnormal height");
    }
}
