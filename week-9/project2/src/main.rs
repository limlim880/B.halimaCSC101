use std::fs::File;
use std::io::Write;

fn main() {
    let students = [
        ("Oluchi Nnordi", "ACC1021111", "Accounting", 300),
        ("Adams Aliyu", "ECO1010101", "Economics", 200),
        ("Sohma Bolade", "CSC1028828", "Computer Science", 400),
        ("Adekunele Gold", "EEE1020220", "Electrical Eng.", 500),
        ("Bianca Edemoh", "MEE1020201", "Mechanical Eng.", 300),
    ];

    println!("PAU SMIS");
    println!("Student Name\tMatric. Number\tDepartment\tLevel");
    for student in &students {
        println!(
            "{}\t{}\t{}\t{}",
            student.0, student.1, student.2, student.3
        );
    }

    let mut file = File::create("students.txt").expect("Failed to create file");
    file.write_all(b"PAU SMIS\n").expect("Write failed");
    file.write_all(b"Student Name\tMatric. Number\tDepartment\tLevel\n")
        .expect("Write failed");
    for student in &students {
        let line = format!(
            "{}\t{}\t{}\t{}\n",
            student.0, student.1, student.2, student.3
        );
        file.write_all(line.as_bytes()).expect("Write failed");
    }

    println!("\nStudent data has been saved to 'students.txt'.");
}
