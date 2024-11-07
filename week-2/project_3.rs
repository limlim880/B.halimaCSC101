fn main() {
	let principal: f64 = 510000.0;
	let rate: f64 = 0.05;
	let time: f64 = 3.0;
	let n: f64 = 1.0;
	let amount = principal * (1.0 - rate / n).powf(n * time);

    println!("The value of the TV after 3 years is: {} ", amount);
   
}