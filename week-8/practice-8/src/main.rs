fn main() {
    // Initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail");
    println!("Original tuple = {:?}", mountain_heights);

    // Change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.1 = 8516; // Corrected index to 1 for the 4th element
    println!("Changed tuple = {:?}", mountain_heights);
}