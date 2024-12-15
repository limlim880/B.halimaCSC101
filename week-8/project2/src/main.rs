use std::io;

fn main() {
    let mut candidates = vec![
        ("Alice", 5),
        ("Bob", 8),
        ("Charlie", 3),
        ("Diana", 10),
        ("Ethan", 7),
    ];

    println!("--- EY Nigeria Developer Experience Checker ---\n");
    println!("Candidates and their years of programming experience:");

    for (name, experience) in &candidates {
        println!("{} - {} years", name, experience);
    }

    let mut max_experience = 0;
    let mut best_candidate = "";

    for (name, experience) in &candidates {
        if *experience > max_experience {
            max_experience = *experience;
            best_candidate = name;
        }
    }

    println!(
        "\nThe candidate with the highest programming experience is: {} with {} years.",
        best_candidate, max_experience
    );

    println!("\nDo you want to add a new candidate? (yes/no): ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    if choice.trim().eq_ignore_ascii_case("yes") {
        println!("Enter the candidate's name: ");
        let mut new_name = String::new();
        io::stdin().read_line(&mut new_name).expect("Failed to read input");

        println!("Enter the years of experience: ");
        let mut experience_input = String::new();
        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read input");
        let new_experience: i32 = experience_input.trim().parse().expect("Invalid number");

        candidates.push((new_name.trim(), new_experience));
        println!("\nUpdated list of candidates:");

        for (name, experience) in &candidates {
            println!("{} - {} years", name, experience);
        }
    } else {
        println!("\nThank you for using the EY Developer Experience Checker!");
    }
}
