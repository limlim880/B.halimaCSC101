fn main() {
    let item1 = ("Toshiba", 2, 450_000.00);
    let item2 = ("Mac", 1, 1_500_000.00);
    let item3 = ("HP", 3, 750_000.00);
    let item4 = ("Dell", 3, 2_850_000.00);
    let item5 = ("Acer", 1, 250_000.00);
    
    // Calculate the total sales by multiplying quantity and amount for each item
    let total_sales = (item1.1 as f64 * item1.2)
                    + (item2.1 as f64 * item2.2)
                    + (item3.1 as f64 * item3.2)
                    + (item4.1 as f64 * item4.2)
                    + (item5.1 as f64 * item5.2);
    
    // Sum up the quantities of ea  ch item
    let total_quantity = (item1.1 + item2.1 + item3.1 + item4.1 + item5.1) as f64;

    // Calculate the average sales per item
    let average_sales = total_sales / total_quantity;

    println!("Total sales: {}", total_sales);
    println!("Average sales per item: {}", average_sales);
}


