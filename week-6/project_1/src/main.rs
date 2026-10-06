use std::io;

// 1. Trapezium Area: height / 2 * (base1 + base2)
fn calculate_trapezium() -> f64 {
    let mut input = String::new();
    
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter base1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter base2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base2: f64 = input.trim().parse().expect("Invalid input");

    height / 2.0 * (base1 + base2)
}

// 2. Rhombus Area: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus() -> f64 {
    let mut input = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagonal1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagonal2: f64 = input.trim().parse().expect("Invalid input");

    0.5 * diagonal1 * diagonal2
}

// 3. Parallelogram Area: base * altitude
fn calculate_parallelogram() -> f64 {
    let mut input = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let altitude: f64 = input.trim().parse().expect("Invalid input");

    base * altitude
}

// 4. Cube Surface Area: 6 * side * side
fn calculate_cube() -> f64 {
    let mut input = String::new();

    println!("Enter side length:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let side: f64 = input.trim().parse().expect("Invalid input");

    6.0 * side * side
}

// 5. Cylinder Volume: pi * radius * radius * height
fn calculate_cylinder() -> f64 {
    let mut input = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let radius: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid input");

    const PI: f64 = std::f64::consts::PI;
    PI * radius * radius * height
}

fn main() {
    println!("=== THE SHAPE CALCULATOR ===");
    println!("Select a shape / calculation:");
    println!("1. Trapezium (Area)");
    println!("2. Rhombus (Area)");
    println!("3. Parallelogram (Area)");
    println!("4. Cube (Surface Area)");
    println!("5. Cylinder (Volume)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => {
            let result = calculate_trapezium();
            println!("The Area of the Trapezium is: {}", result);
        }
        2 => {
            let result = calculate_rhombus();
            println!("The Area of the Rhombus is: {}", result);
        }
        3 => {
            let result = calculate_parallelogram();
            println!("The Area of the Parallelogram is: {}", result);
        }
        4 => {
            let result = calculate_cube();
            println!("The Surface Area of the Cube is: {}", result);
        }
        5 => {
            let result = calculate_cylinder();
            println!("The Volume of the Cylinder is: {}", result);
        }
        _ => {
            println!("Invalid choice! Please restart the program and select a number from 1 to 5.");
        }
    }
}