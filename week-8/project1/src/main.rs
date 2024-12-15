use std::io;

fn main() {
    let aps_levels = vec![
        "APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES",
    ];

    let office_admin_roles = vec![
        "Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO",
    ];

    let academic_roles = vec![
        "-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean",
    ];

    let lawyer_roles = vec![
        "Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner",
    ];

    let teacher_roles = vec![
        "Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal",
    ];

    println!("--- Public Service APS Level Checker ---");
    println!("Enter your role title (e.g., 'Associate', 'Leading Teacher', etc.): ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let role = input.trim();

    let mut found = false;

    for i in 0..aps_levels.len() {
        if office_admin_roles[i] == role 
            || academic_roles[i] == role
            || lawyer_roles[i] == role
            || teacher_roles[i] == role 
        {
            println!(
                "\nRole '{}' is classified under APS Level: {}",
                role, aps_levels[i]
            );
            found = true;
            break;
        }
    }

    if !found {
        println!("\nRole '{}' is not found in the APS table.", role);
    }
}
