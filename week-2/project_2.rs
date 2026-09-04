fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_00.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;

	let sum = toshiba + mac + hp + dell + acer;
	println!("Sum of sales is {}",sum );
	let average = sum / 5.0;
	println!("Average of sales is {}",average);

}