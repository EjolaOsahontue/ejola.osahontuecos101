use std::io;

// Function to calculate the area of a trapezium
fn area_of_trapezium(base1: f32, base2: f32, height: f32) -> f32 {
    height / 2.0 * (base1 + base2)
}

// Function to calculate the area of a rhombus
fn area_of_rhombus(diagonal1: f32, diagonal2: f32) -> f32 {
    0.5 * diagonal1 * diagonal2
}

// Function to calculate the area of a parallelogram
fn area_of_parallelogram(base: f32, altitude: f32) -> f32 {
    base * altitude
}

// Function to calculate the area of a cube
fn area_of_cube(side_length: f32) -> f32 {
    6.0 * side_length * side_length
}

// Function to calculate the volume of a cylinder
fn volume_of_cylinder(radius: f32, height: f32) -> f32 {
    std::f32::consts::PI * radius.powi(2) * height
}

fn main() {
    println!("Choose an equation to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: usize = choice.trim().parse().expect("Invalid choice");

    match choice {
        1 => {
            println!("Enter base1: ");
            let base1 = get_user_input();
            println!("Enter base2: ");
            let base2 = get_user_input();
            println!("Enter height: ");
            let height = get_user_input();
            let result = area_of_trapezium(base1, base2, height);
            println!("Area of Trapezium: {}", result);
        }
        2 => {
            println!("Enter diagonal1: ");
            let diagonal1 = get_user_input();
            println!("Enter diagonal2: ");
            let diagonal2 = get_user_input();
            let result = area_of_rhombus(diagonal1, diagonal2);
            println!("Area of Rhombus: {}", result);
        }
        3 => {
            println!("Enter base: ");
            let base = get_user_input();
            println!("Enter altitude: ");
            let altitude = get_user_input();
            let result = area_of_parallelogram(base, altitude);
            println!("Area of Parallelogram: {}", result);
        }
        4 => {
            println!("Enter side length: ");
            let side_length = get_user_input();
            let result = area_of_cube(side_length);
            println!("Area of Cube: {}", result);
        }
        5 => {
            println!("Enter radius: ");
            let radius = get_user_input();
            println!("Enter height: ");
            let height = get_user_input();
            let result = volume_of_cylinder(radius, height);
            println!("Volume of Cylinder: {}", result);
        }
        _ => println!("Invalid choice"),
    }
}

// Function to get user input as f32
fn get_user_input() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}
