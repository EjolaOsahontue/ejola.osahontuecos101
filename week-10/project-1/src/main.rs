struct Laptop {
    brand: String,
    cost: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.cost * quantity
    }
}

fn main() {
    let laptops = vec![
        Laptop { brand: "HP".to_string(), cost: 650000 },
        Laptop { brand: "IBM".to_string(), cost: 755000 },
        Laptop { brand: "Toshiba".to_string(), cost: 550000 },
        Laptop { brand: "Dell".to_string(), cost: 850000 },
    ];

    let mut total_cost = 0;

    for laptop in &laptops {
        total_cost += laptop.total_cost(3);
    }

    println!("Total cost: {}", total_cost);
}
