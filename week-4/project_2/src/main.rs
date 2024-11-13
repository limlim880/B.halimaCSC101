use std::io;

fn main() {
    // Get experience input from user
    println!("Is this employee experienced? (Yes/No)");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input.");
    let is_experienced = experience_input.trim().eq_ignore_ascii_case("yes");

    // Get age input from user
    println!("What is this employee's age?");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input.");
    let age: i32 = age_input.trim().parse().expect("Please enter a valid number.");

    // Determine the incentive
    let incentive;

    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else {
            incentive = 1_300_000; // This now covers all ages below 30
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive for the employee is ₦{}", incentive);
}
