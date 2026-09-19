use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter the value of c:");
    io::stdin().read_line(&mut input).unwrap();
    let c: f64 = input.trim().parse().unwrap();

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("The roots are:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);

        println!("There is one real root:");
        println!("Root = {}", root);
    } else {
        println!("There are no real roots.");
    }
}