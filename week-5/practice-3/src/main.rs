use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    
    let base: f32 = input1.trim().parse().expect("Input not a valid number");
    
    println!("Enter height: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    
    let height: f32 = input2.trim().parse().expect("Input not a valid number");
    
    if base > 0.0 && height > 0.0 {
        let area: f32 = (base * height) / 2.0;
        println!("Area of a triangle: {}", area);
    } else {
        println!("Base and height must be greater than 0.");
    }
}
