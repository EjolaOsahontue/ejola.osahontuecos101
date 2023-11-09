use std::io;

fn main() {
    // Define menu items and their prices
    let menu = vec![
        ("P", "Poundo Yam/Edikaiko Soup", 3200.0),
        ("F", "Fried Rice & Chicken", 3000.0),
        ("A", "Amala & Ewedu Soup", 2500.0),
        ("E", "Eba & Egusi Soup", 2000.0),
        ("W", "White Rice & Stew", 2500.0),
    ];

    println!("Welcome to the Food Ordering System!");
    println!("Menu:");

    for (code, item, price) in menu.iter() {
        println!("{}. {} - N{:.2}", code, item, price);
    }

    // Input food items and quantities
    let mut order: Vec<(String, f64)> = Vec::new();
    let mut total: f64 = 0.0;

    loop {
        println!("Enter the item code (0 to finish) and quantity:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut parts = input.trim().split_whitespace();
        let item_code = match parts.next() {
            Some(s) => s.to_uppercase(),
            None => break,
        };
        if item_code == "0" {
            break;
        }

        let quantity: f64 = match parts.next() {
            Some(s) => s.parse().expect("Invalid input"),
            None => break,
        };

        let menu_item = menu.iter().find(|&&(code, _, _)| code == item_code);
        match menu_item {
            Some(&(_, food, price)) => {
                let item_total = price * quantity;
                total += item_total;
                order.push((food.to_string(), item_total));
            }
            None => {
                println!("Invalid item code. Please choose a valid item.");
                continue;
            }
        }
    }

    // Calculate total charges
    let discount = if total > 10_000.0 { 0.05 } else { 0.0 };
    let discounted_total = total - (total * discount);

    // Display the order and total charges
    println!("Order Summary:");
    for (food, item_total) in order.iter() {
        println!("{}: N{:.2}", food, item_total);
    }
    println!("Total Charges: N{:.2}", discounted_total);

    if discount > 0.0 {
        println!("Discount Applied: {:.0}% (N{:.2})", discount * 100.0, total * discount);
    }
}
