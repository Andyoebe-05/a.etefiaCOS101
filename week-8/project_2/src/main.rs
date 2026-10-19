fn main() {
    // Vector of tuples: (name, years of experience)
    let developers = vec![
        ("Grace", 3),
        ("John", 5),
        ("Mary", 2),
        ("David", 7),
        ("Sarah", 4),
    ];

    // Assume the first developer has the highest experience
    let mut highest_name = developers[0].0;
    let mut highest_years = developers[0].1;

    // Check the remaining developers
    for i in 1..developers.len() {
        if developers[i].1 > highest_years {
            highest_years = developers[i].1;
            highest_name = developers[i].0;
        }
    }

    // Display the result
    println!("Developer with the highest experience:");
    println!("Name: {}", highest_name);
    println!("Years of experience: {}", highest_years);
}