use std::io;

fn main() {
    println!("PAU Cafe App");
    println!("Choose from the MENU!!");
    println!("P - Pounded Yam & Edikaikong Soup - ₦3,200");
    println!("F - Fried Rice & Chicken        - ₦3,000");
    println!("A - Amala & Ewedu Soup          - ₦2,500");
    println!("E - Eba & Egusi Soup            - ₦2,000");
    println!("W - White Rice & Stew            - ₦2,500");

    // Get food type from customer
    let mut food_type = String::new();
    println!("\nEnter food type (P, F, A, E, W):");
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    // Get quantity from customer
    let mut quantity = String::new();
    println!("Enter quantity:");
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: i32 = quantity
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Determine the price
    let price: f64 = match food_type.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    // Calculate total
    let total = price * quantity as f64;

    // Apply discount if total is greater than ₦10,000
    let discount = if total > 5000.0 {
        total * 0.05
    } else {
        0.0
    };

    let final_total = total - discount;

    // Display results
    println!("\nHERE'S YOUR ORDER SUMMARY");
    println!("Food type: {}", food_type);
    println!("Quantity: {}", quantity);
    println!("Total: ₦{:.2}", total);
    println!("Discount: ₦{:.2}", discount);
    println!("Amount to pay: ₦{:.2}", final_total);
}