fn main() {
	let principal: f64 = 520000000.0;
	let rate: f64 = 0.1;
	let time: f64 = 5.0;
	let n: f64 = 1.0;
	let amount = principal * (1.0 + rate / n).powf(n * time);
    let compound_interest = amount - principal;

    println!("The compound interest after 5 years is: {:.2}", compound_interest);
    println!("The total amount after 5 years is: {}", amount);
}