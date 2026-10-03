use std::io;

fn main() {
    println!("PAU Cafe App");
    println!("Choose from the MENU!!");

    println!("P - Pounded Yam & Edikaikong Soup - ₦3,200");
    println!("F - Fried Rice & Chicken       - ₦3,000");
    println!("A - Amala & Ewedu Soup         - ₦2,500");
    println!("E - Eba & Egusi Soup           - ₦2,000");
    println!("W - White Rice & Stew          - ₦2,500");

    // Get food choices
    let mut food_input = String::new();

    println!("\nEnter food type(s), e.g. P & F:");
    io::stdin()
        .read_line(&mut food_input)
        .expect("Failed to read input");

    // Get quantity
    let mut quantity_input = String::new();

    println!("Enter quantity:");
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let quantity: i32 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a valid quantity");

    let mut total = 0.0;
    let mut valid_choice = false;

    // Check every character entered
    for food in food_input.to_uppercase().chars() {

        let price = match food {
            'P' => {
                valid_choice = true;
                3200.0
            }

            'F' => {
                valid_choice = true;
                3000.0
            }

            'A' => {
                valid_choice = true;
                2500.0
            }

            'E' => {
                valid_choice = true;
                2000.0
            }

            'W' => {
                valid_choice = true;
                2500.0
            }

            _ => 0.0,
        };

        total += price * quantity as f64;
    }

    if !valid_choice {
        println!("Invalid food type!");
        return;
    }

    // Number of meals ordered
    let number_of_food_types = food_input
        .to_uppercase()
        .chars()
        .filter(|c| matches!(c, 'P' | 'F' | 'A' | 'E' | 'W'))
        .count();

    let total_meals = number_of_food_types as i32 * quantity;

    // 10% discount for 3 or more meals
    let discount = if total_meals >= 3 {
        total * 0.10
    } else {
        0.0
    };

    let final_total = total - discount;

    println!("\n===== ORDER SUMMARY =====");
    println!("Quantity of each food: {}", quantity);
    println!("Total meals: {}", total_meals);
    println!("Total: ₦{:.2}", total);
    println!("Discount: ₦{:.2}", discount);
    println!("Amount to pay: ₦{:.2}", final_total);
}