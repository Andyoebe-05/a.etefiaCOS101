fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The City vector has element {}.", city.len());

    // Push new elements into vector
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}", count);
        std::io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input");

        let new_city = input2.trim().to_string();
        city.push(new_city);
    }

    println!("Your preferred cities are:\n");
    let mut count = 1;

    // loop to iterate elements in vector
    for i in city {
        println!("{} {}", count, i);
        count += 1;
    }
}