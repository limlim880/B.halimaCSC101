use std::io;

fn main() {
    let mut total_cost = 0;

    loop {
        println!("Menu:");
        println!("P = Poundo Yam/Edinkaiko Soup - ₦3,200");
        println!("F = Fried Rice & Chicken - ₦3,000");
        println!("A = Amala & Ewedu Soup - ₦2,500");
        println!("E = Eba & Egusi Soup - ₦2,000");
        println!("W = White Rice & Stew - ₦2,500");

        let mut food_type = String::new();
        println!("Enter the food type (P, F, A, E, W):");
        io::stdin()
            .read_line(&mut food_type)
            .expect("Failed to read input.");
        let food_type = food_type.trim().to_uppercase();

        let price = if food_type == "P" {
            3200
        } else if food_type == "F" {
            3000
        } else if food_type == "A" {
            2500
        } else if food_type == "E" {
            2000
        } else if food_type == "W" {
            2500
        } else {
            println!("Invalid food type!");
            continue;
        };

        let mut quantity_input = String::new();
        println!("Enter the quantity:");
        io::stdin().read_line(&mut quantity_input).expect("Failed to read input.");
        let quantity: u32 = quantity_input.trim().parse().expect("Please enter a valid number");

        let item_cost = price * quantity;
        total_cost += item_cost;

        let mut another_item = String::new();
        println!("Do you want to order another item? (yes or no):");
        io::stdin()
            .read_line(&mut another_item)
            .expect("Failed to read input.");
        
        if another_item.trim().to_lowercase() != "yes" {
            break;
        }
    }

    if total_cost > 10_000 {
        total_cost = ((total_cost as f64 * 0.95).round()) as u32;
        println!("A 5% discount has been applied.");
    }

    println!("Final total cost: ₦{}", total_cost);
}
