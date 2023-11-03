fn main() {
    let distance1_miles = 80.0;
    let time1_hours = 2.0;
    let speed1_mph = distance1_miles / time1_hours;
    let speed1_kmph = speed1_mph * 1.60934;

    let distance2_miles = 120.0;
    let time2_hours = 4.0;
    let speed2_mph = distance2_miles / time2_hours;
    let speed2_kmph = speed2_mph * 1.60934;

    println!("Scenario 1: Speed in km/h: {:.2}", speed1_kmph);
    println!("Scenario 2: Speed in km/h: {:.2}", speed2_kmph);
}
