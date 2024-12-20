use std::io;

fn main() {
    println!("Enter the number of siblings:");
    let mut sibling_count_input = String::new();
    io::stdin().read_line(&mut sibling_count_input).expect("Failed to read input.");
    let sibling_count: u32 = sibling_count_input.trim().parse().expect("Please enter a valid number.");

    for i in 0..sibling_count {
        println!("\nEntering information for sibling {}:", i + 1);

        println!("Enter siblings name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input.");
        let name = name.trim();

        println!("Enter siblings age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input.");
        let age: u32 = age_input.trim().parse().expect("Please enter a valid age.");

        println!("Enter siblings gender:");
        let mut gender = String::new();
        io::stdin().read_line(&mut gender).expect("Failed to read input.");
        let gender = gender.trim();

        println!("Enter siblings country:");
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Failed to read input.");
        let country = country.trim();

        println!("Enter the city where the siblings family currently lives:");
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input.");
        let city = city.trim();

        let mut marital_status = String::new();
        let mut number_of_children = 0;
        let mut employment_status = String::new();
        let mut university_name = String::new();
        let mut course_of_study = String::new();
        let mut year_of_study = 0;
        let mut studying_abroad = false;
        let mut study_country = String::new();
        let mut children_info = Vec::new();
        let mut partner_name = String::new();
        let mut relationship_years = 0;
        let mut live_together = false;
        let mut live_city = String::new();
        let mut completed_waec = false;
        let mut secondary_school = String::new();
        let mut final_grade = String::new();
        let mut current_grade = String::new();
        let mut planning_waec = false;
        let mut waec_year = 0;

        if age >= 18 {
            println!("Is your sibling married/single/in a relationship?");
            io::stdin().read_line(&mut marital_status).expect("Failed to read input.");
            marital_status = marital_status.trim().to_string();

            if marital_status.eq_ignore_ascii_case("married") {
                println!("How many children do they have?");
                let mut children_input = String::new();
                io::stdin().read_line(&mut children_input).expect("Failed to read input.");
                number_of_children = children_input.trim().parse().expect("Please enter a valid number.");

                for j in 0..number_of_children {
                    println!("\nEntering information for child {} of sibling {}:", j + 1, i + 1);

                    println!("Enter childs name:");
                    let mut child_name = String::new();
                    io::stdin().read_line(&mut child_name).expect("Failed to read input.");
                    let child_name = child_name.trim();

                    println!("Enter childs age:");
                    let mut child_age_input = String::new();
                    io::stdin().read_line(&mut child_age_input).expect("Failed to read input.");
                    let child_age: u32 = child_age_input.trim().parse().expect("Please enter a valid age.");

                    println!("Enter childs school name:");
                    let mut school_name = String::new();
                    io::stdin().read_line(&mut school_name).expect("Failed to read input.");
                    let school_name = school_name.trim();

                    children_info.push((child_name.to_string(), child_age, school_name.to_string()));
                }
            } else if marital_status.eq_ignore_ascii_case("single") {
                println!("Are they a student or employed?");
                io::stdin().read_line(&mut employment_status).expect("Failed to read input.");
                employment_status = employment_status.trim().to_string();

                if employment_status.eq_ignore_ascii_case("student") {
                    println!("Enter the university name:");
                    io::stdin().read_line(&mut university_name).expect("Failed to read input.");
                    university_name = university_name.trim().to_string();

                    println!("Enter the course of study:");
                    io::stdin().read_line(&mut course_of_study).expect("Failed to read input.");
                    course_of_study = course_of_study.trim().to_string();

                    println!("Enter the year of study:");
                    let mut year_input = String::new();
                    io::stdin().read_line(&mut year_input).expect("Failed to read input.");
                    year_of_study = year_input.trim().parse().expect("Please enter a valid year.");

                    println!("Are they studying in their home country or abroad?");
                    let mut study_location = String::new();
                    io::stdin().read_line(&mut study_location).expect("Failed to read input.");
                    study_location = study_location.trim().to_string();

                    studying_abroad = study_location.eq_ignore_ascii_case("abroad");
                    if studying_abroad {
                        println!("Enter the country they are studying in:");
                        io::stdin().read_line(&mut study_country).expect("Failed to read input.");
                        study_country = study_country.trim().to_string();
                    }
                }
            } else if marital_status.eq_ignore_ascii_case("in a relationship") {
                println!("Enter their partner's name:");
                io::stdin().read_line(&mut partner_name).expect("Failed to read input.");
                partner_name = partner_name.trim().to_string();

                println!("How many years have they been dating?");
                let mut years_input = String::new();
                io::stdin().read_line(&mut years_input).expect("Failed to read input.");
                relationship_years = years_input.trim().parse().expect("Please enter a valid number.");

                println!("Do they live together? (yes/no)");
                let mut live_input = String::new();
                io::stdin().read_line(&mut live_input).expect("Failed to read input.");
                live_together = live_input.trim().eq_ignore_ascii_case("yes");

                if live_together {
                    println!("Enter the city they live in together:");
                    io::stdin().read_line(&mut live_city).expect("Failed to read input.");
                    live_city = live_city.trim().to_string();
                }
            }
        } else {
            println!("Have they completed WAEC? (yes/no)");
            let mut waec_input = String::new();
            io::stdin().read_line(&mut waec_input).expect("Failed to read input.");
            completed_waec = waec_input.trim().eq_ignore_ascii_case("yes");

            if completed_waec {
                println!("Enter the name of their secondary school:");
                io::stdin().read_line(&mut secondary_school).expect("Failed to read input.");
                secondary_school = secondary_school.trim().to_string();

                println!("Enter their final grade:");
                io::stdin().read_line(&mut final_grade).expect("Failed to read input.");
                final_grade = final_grade.trim().to_string();
            } else {
                println!("What grade are they currently in?");
                io::stdin().read_line(&mut current_grade).expect("Failed to read input.");
                current_grade = current_grade.trim().to_string();

                println!("Are they planning to take WAEC soon? (yes/no)");
                let mut planning_input = String::new();
                io::stdin().read_line(&mut planning_input).expect("Failed to read input.");
                planning_waec = planning_input.trim().eq_ignore_ascii_case("yes");

                if planning_waec {
                    println!("What year are they planning to take WAEC?");
                    let mut waec_year_input = String::new();
                    io::stdin().read_line(&mut waec_year_input).expect("Failed to read input.");
                    waec_year = waec_year_input.trim().parse().expect("Please enter a valid year.");
                }
            }
        }

        println!("\n--- Sibling {} Information ---", i + 1);
        println!("Name: {}", name);
        println!("Age: {}", age);
        println!("Gender: {}", gender);
        println!("Country: {}", country);
        println!("City: {}", city);

        if age >= 18 {
            println!("Marital Status: {}", marital_status);
            if marital_status.eq_ignore_ascii_case("married") {
                println!("Number of Children: {}", number_of_children);
                for (index, (child_name, child_age, school_name)) in children_info.iter().enumerate() {
                    println!(
                        "Child {}: Name: {}, Age: {}, School: {}",
                        index + 1,
                        child_name,
                        child_age,
                        school_name
                    );
                }
            } else if marital_status.eq_ignore_ascii_case("single") && employment_status.eq_ignore_ascii_case("student") {
                println!("University: {}", university_name);
                println!("Course of Study: {}", course_of_study);
                println!("Year of Study: {}", year_of_study);
                if studying_abroad {
                    println!("Studying Abroad in: {}", study_country);
                } else {
                    println!("Studying in Home Country");
                }
            } else if marital_status.eq_ignore_ascii_case("in a relationship") {
                println!("Partners Name: {}", partner_name);
                println!("Years of Relationship: {}", relationship_years);
                if live_together {
                    println!("Living Together in: {}", live_city);
               
