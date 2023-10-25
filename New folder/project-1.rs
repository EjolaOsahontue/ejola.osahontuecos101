
fn main() {
    // Define the principal amount, rate, time, and number of times compounded annually.
    let principal = 520_000_000.0;  // Naira
    let rate = 10.0;  // 10% annual interest rate
    let time = 5;  // 5 years
    let n = 1;  // Compounded annually

    // Calculate compound interest using the formula: CI = A - P
    let r = rate / 100.0;  // Convert rate to a decimal
    let a = principal * (1.0 + r / n as f64).powi(n * time);
    let ci = a - principal;

    // Print the result
    println!("The compound interest is: N{:.*}", 2, ci);
}
