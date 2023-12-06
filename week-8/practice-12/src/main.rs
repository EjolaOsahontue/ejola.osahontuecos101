fn main() {
    // Mutable array
    let mut colors = ["red", "green", "yellow", "white"];
    println!("\nOriginal array = {:?}", colors);

    // Mutable slice
    let sliced_colors = &mut colors[1..3];
    println!("First slice = {:?}", sliced_colors);

    // Change the value of the original array at the first index of the slice
    sliced_colors[0] = "purple";
    println!("Changed slice = {:?}", sliced_colors);
}