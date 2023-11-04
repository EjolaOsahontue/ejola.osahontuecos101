use std::io;

fn main() {
    // Input distance in miles
    let mut input_distance = String::new();
    println!("Enter the distance in miles: ");
    io::stdin().read_line(&mut input_distance).expect("Failed to read input");
    let distance_miles: f32 = input_distance.trim().parse().expect("Invalid input for distance");

    // Input time in hours
    let mut input_time = String::new();
    println!("Enter the time in hours: ");
    io::stdin().read_line(&mut input_time).expect("Failed to read input");
    let time_hours: f32 = input_time.trim().parse().expect("Invalid input for time");

    // Calculate speed in kilometers per hour
    let distance_kilometers = distance_miles * 1.60934; // Convert miles to kilometers
    let speed_kph = distance_kilometers / time_hours;

    println!("The car is traveling at {:.2} kilometers per hour.", speed_kph);
}
