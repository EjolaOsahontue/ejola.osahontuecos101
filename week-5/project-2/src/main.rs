use std::io;

fn main() {
    println!("Employee Incentive Determiner");

    // Input the employee's experience and age
    println!("Enter 'experienced' or 'inexperienced': ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    println!("Enter age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age = age.trim().parse::<u32>().expect("Invalid age input");

    // Calculate and display the incentive
    let incentive = match (experience.as_str(), age) {
        ("experienced", age) if age >= 40 => "N1,560,000",
        ("experienced", age) if age >= 30 => "N1,480,000",
        ("experienced", age) if age >= 28 => "N1,300,000",
        ("experienced", age) if age < 18 => "Nothing for you, get older first",
        ("experienced", _) => "Nada",
        ("inexperienced", _) => "N100,000",
        _ => "Nada",
    };

    // Display the incentive
    println!("Incentive: {}", incentive);
}
