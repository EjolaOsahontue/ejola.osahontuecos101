use std::io;

fn main() {
    loop {
        println!("Choose an equation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice: u32 = get_user_input("Enter your choice: ");

        match choice {
            1 => calculate_trapezium_area(),
            2 => calculate_rhombus_area(),
            3 => calculate_parallelogram_area(),
            4 => calculate_cube_area(),
            5 => calculate_cylinder_volume(),
            6 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please choose a number between 1 and 6."),
        }
    }
}

fn get_user_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn calculate_trapezium_area() {
    let height = get_user_input("Enter the height: ");
    let base1 = get_user_input("Enter the first base length: ");
    let base2 = get_user_input("Enter the second base length: ");

    let area = height as f64 / 2.0 * (base1 + base2) as f64;
    println!("The area of the trapezium is: {}", area);
}

fn calculate_rhombus_area() {
    let diagonal1 = get_user_input("Enter the length of the first diagonal: ");
    let diagonal2 = get_user_input("Enter the length of the second diagonal: ");

    let area = 0.5 * diagonal1 as f64 * diagonal2 as f64;
    println!("The area of the rhombus is: {}", area);
}

fn calculate_parallelogram_area() {
    let base = get_user_input("Enter the base length: ");
    let altitude = get_user_input("Enter the altitude length: ");

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

fn calculate_cube_area() {
    let side_length = get_user_input("Enter the length of the side: ");

    let area = 6 * side_length * side_length;
    println!("The area of the cube is: {}", area);
}

fn calculate_cylinder_volume() {
    let radius = get_user_input("Enter the radius of the cylinder: ");
    let height = get_user_input("Enter the height of the cylinder: ");

    let volume = std::f64::consts::PI * (radius as f64).powi(2) * height as f64;
    println!("The volume of the cylinder is: {}", volume);
}
