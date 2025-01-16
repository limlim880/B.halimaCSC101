struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

impl Laptop {
    fn calculate_total_cost(&self, purchase_quantity: u32) -> u32 {
        self.price * purchase_quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: "HP".to_string(),
        price: 650_000,
        quantity: 10,
    };
    let ibm = Laptop {
        brand: "IBM".to_string(),
        price: 755_000,
        quantity: 6,
    };
    let toshiba = Laptop {
        brand: "Toshiba".to_string(),
        price: 550_000,
        quantity: 10,
    };
    let dell = Laptop {
        brand: "Dell".to_string(),
        price: 850_000,
        quantity: 4,
    };

    let total_cost = hp.calculate_total_cost(3)
        + ibm.calculate_total_cost(3)
        + toshiba.calculate_total_cost(3)
        + dell.calculate_total_cost(3);

    println!("The total cost for purchasing 3 laptops from each brand is: ₦{}", total_cost);
}
