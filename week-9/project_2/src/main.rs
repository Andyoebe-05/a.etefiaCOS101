
use std::io::Write;

// PAU Student Management Information System

fn main() {
    // Step 1: Store student records in a vector
    let students: Vec<(&str, &str, &str, i32)> = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Bianca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // Step 2: Display student records on screen
    println!("PAU STUDENT MANAGEMENT INFORMATION SYSTEM");
    println!("{:<20} {:<15} {:<15} {}", 
        "Student Name", "Matric Number", "Department", "Level");

    for student in &students {
        println!("{:<20} {:<15} {:<15} {}",
            student.0, student.1, student.2, student.3);
    }

    // Step 3: Write student records to a file
    let mut file = std::fs::File::create("PAU_SMIS.txt")
        .expect("Cannot create file");

    file.write_all(
        b"PAU STUDENT MANAGEMENT INFORMATION SYSTEM\n"
    ).expect("Write failed");

    file.write_all(
        b"Student Name | Matric Number | Department | Level\n"
    ).expect("Write failed");

    for student in &students {
        let record = format!(
            "{} | {} | {} | {}\n",
            student.0, student.1, student.2, student.3
        );

        file.write_all(record.as_bytes())
            .expect("Write failed");
    }

    println!("\nStudent records saved to PAU_SMIS.txt");
}
