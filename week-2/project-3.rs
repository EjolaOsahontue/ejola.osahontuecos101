fn main() {
    let initial_value: f64 = 210000.0;
    let depreciation_rate: f64 = 5.0;
    let years: i32 = 3;
    let value_after_depreciation = (initial_value) * (1.0 - (depreciation_rate / 100.0)).powi(years);
    println!("Value of the TV after {} years: N{:.2}", years, value_after_depreciation);
}
