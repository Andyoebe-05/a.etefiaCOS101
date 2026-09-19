use std::io;

fn main() {
    let mut experience = String::new();
    let mut age_input = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin().read_line(&mut experience).unwrap();

    println!("Enter your age:");
    io::stdin().read_line(&mut age_input).unwrap();

    let age: i32 = age_input.trim().parse().unwrap();

    if experience.trim().to_lowercase() == "yes" {
        if age >= 40 {
            println!("Annual incentive: ₦1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Annual incentive: ₦1,300,000");
        } else {
            println!("Age does not fall within the specified criteria.");
        }
    } else {
        println!("Annual incentive: ₦100,000");
    }
}