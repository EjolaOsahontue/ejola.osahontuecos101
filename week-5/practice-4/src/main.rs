use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    let age: i32 = input2.trim().parse().expect("Input not a valid number");

    if age >= 18 {
        println!("Welcome to the party, {}!", input1);
    } else {
        println!("Oops, you are not of age to enter the party, {}.", input1);
    }
}
