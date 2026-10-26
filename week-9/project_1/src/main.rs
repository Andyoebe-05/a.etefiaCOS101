
use std::io::{Read, Write};

// Nigerian Breweries Drink Category

fn main() {
    let mut file = std::fs::File::create("Nigerian_Breweries.csv")
        .expect("Create failed");

    file.write_all(
        b"Lager,Stout,Non-Alcoholic\n"
    ).expect("Write failed");

    file.write_all(
        b"33 Export,Legend,Maltina\nDesperados,Turbo King,Amstel Malta\nGoldberg,Williams,Malta Gold\nGulder,Fayrouz\nHeineken,\nStar,\n"
    ).expect("Write failed");

    println!("Data written to CSV file.");

    drop(file);

    let mut file = std::fs::File::open("Nigerian_Breweries.csv")
        .expect("Cannot open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Read failed");

    println!("\n{}", contents);
}
