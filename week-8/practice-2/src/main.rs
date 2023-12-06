use std::io;

fn main() {
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];
    let mut input = String::new();

    println!("Enter an index value between 0 and 7:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Index is the non-negative value which is smaller than the size of the vector
    let index: usize = input.trim().parse().expect("Invalid input");

    // Getting the value at the given index
    let ch: char = v[index];

    println!("{} is the character for index {}\n", ch, index);
}