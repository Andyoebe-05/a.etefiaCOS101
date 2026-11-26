use std::fs;
use std::io;

fn show_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Could not read {}: {}", path, e),
    }
}

fn administrator() {
    show_file("admin_db.sql");
}

fn project_manager() {
    show_file("project_tb.sql");
}

fn employee() {
    show_file("staff_tb.sql");
}

fn customer() {
    show_file("customer_tb.sql");
}

fn vendor() {
    show_file("dataplan_tb.sql");
}

fn main() {
    println!("Who is asking?");
    println!("(administrator / project manager / employee / customer / vendor)");

    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();

    match role.trim().to_lowercase().as_str() {
        "administrator" => administrator(),
        "project manager" => project_manager(),
        "employee" => employee(),
        "customer" => customer(),
        "vendor" => vendor(),
        _ => println!("Unknown role. Access denied."),
    }
}