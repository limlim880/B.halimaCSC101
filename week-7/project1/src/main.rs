use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        println!("Enter your choice (1-6):");
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice = choice.trim();

        if choice == "1" {
            println!("Enter height:");
            let height = read_input();
            println!("Enter base1:");
            let base1 = read_input();
            println!("Enter base2:");
            let base2 = read_input();

            println!("Area of Trapezium: {}", (height / 2.0) * (base1 + base2));
        } else if choice == "2" {
            println!("Enter diagonal1:");
            let diagonal1 = read_input();
            println!("Enter diagonal2:");
            let diagonal2 = read_input();

            println!("Area of Rhombus: {}", 0.5 * diagonal1 * diagonal2);
        } else if choice == "3" {
            println!("Enter base:");
            let base = read_input();
            println!("Enter altitude:");
            let altitude = read_input();

            println!("Area of Parallelogram: {}", base * altitude);
        } else if choice == "4" {
            println!("Enter the length of a side:");
            let side = read_input();

            println!("Area of Cube: {}", 6.0 * side * side);
        } else if choice == "5" {
            println!("Enter radius:");
            let radius = read_input();
            println!("Enter height:");
            let height = read_input();

            println!("Volume of Cylinder: {}", 3.14159 * radius * radius * height);
        } else if choice == "6" {
            println!("Exiting program. Goodbye!");
            break;
        } else {
            println!("Invalid choice. Please try again.");
        }
    }
}

fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().parse().expect("Please enter a valid number.")
}
