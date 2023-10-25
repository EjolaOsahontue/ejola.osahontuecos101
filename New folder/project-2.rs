fn main() {
    let sales = [
        ("Toshiba", 2, 450000.0),
        ("Mac", 1, 1500000.0),
        ("HP", 3, 750000.0),
        ("Dell", 3, 2850000.0),
        ("Acer", 1, 250000.0),
    ];

    let mut total_sales = 0.0;

    for sale in &sales {
        total_sales += sale.2;
    }

    let average_sales = total_sales / sales.len() as f64;

    println!("Total sales: {:.2}", total_sales);
    println!("Average sales: {:.2}", average_sales);
}