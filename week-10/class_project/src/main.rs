struct Laptop {
    brand: String,
    stock: u64,
    price: u64,
}

impl Laptop {
    fn cost_of(&self, qty: u64) -> u64 {
        self.price * qty
    }

    // &mut self: changes the laptop. Takes the sold laptops out of stock
    fn sell(&mut self, qty: u64) {
        self.stock -= qty;
    }
}

// Borrows the whole vector with &, so main keeps ownership of shop
fn show_stock(shop: &Vec<Laptop>) {
    println!();
    println!("Stock left after the sale:");
    for laptop in shop {
        println!("{:<10} {}", laptop.brand, laptop.stock);
    }
}

fn main() {
    // The 30 laptops Mr Ogbeifuna received
    let mut shop = vec![
        Laptop { brand: String::from("HP"), stock: 10, price: 650_000 },
        Laptop { brand: String::from("IBM"), stock: 6, price: 755_000 },
        Laptop { brand: String::from("Toshiba"), stock: 10, price: 550_000 },
        Laptop { brand: String::from("Dell"), stock: 4, price: 850_000 },
    ];

    let qty_each: u64 = 3; // the customer buys 3 from each brand
    let mut total: u64 = 0;
    let mut laptops_sold: u64 = 0;

    println!("ALABA ELECTRONICS: PURCHASE RECEIPT");
    println!("{:<10} {:>5} {:>12} {:>14}", "Brand", "Qty", "Price each", "Cost");
    println!("--------------------------------------------");

    // &mut shop: borrow the vector so each laptop can be changed
    for laptop in &mut shop {
        if laptop.stock >= qty_each {
            let cost = laptop.cost_of(qty_each);
            laptop.sell(qty_each);
            total += cost;
            laptops_sold += qty_each;
            println!("{:<10} {:>5} {:>12} {:>14}", laptop.brand, qty_each, laptop.price, cost);
        } else {
            println!("{:<10} not enough stock", laptop.brand);
        }
    }

    println!("--------------------------------------------");
    println!("Laptops bought: {}", laptops_sold);
    println!("TOTAL COST: N{}", total);

    show_stock(&shop); // lend shop to the function with &
}