use std::io;

fn main() {
    // Accept user input for the value of n
    let n: u32 = get_input("Enter the value of n: ")
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    // Display the multiplication table vertically
    for i in 1..=n {
        for j in 1..=10 {
            let result = i * j;
            println!("{} x {} = {}", i, j, result);
        }
        println!("------------------------");
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}