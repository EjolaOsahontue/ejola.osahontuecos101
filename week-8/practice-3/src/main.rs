fn value(n: Option<&char>) {
    println!("Element of vector {:?}", n);
}

fn main() {
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

    let mut input1 = String::new();
    println!("\nEnter an index value between 0 and 7:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Index is the non-negative value which is smaller than the size of the vector
    let index: usize = input1.trim().parse().expect("Invalid input");

    // Getting value at given index
    let ch: Option<&char> = v.get(index);
    value(ch);
}