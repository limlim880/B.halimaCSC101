use std::io;
fn main() {

    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

println!("Enter a value of a:");
io::stdin().read_line(&mut input_a).expect("FAILED TO READ INPUT");
let a:f64 = input_a.trim().parse().expect("Please enter a valid number:");

println!("Enter a value of b:");
io::stdin().read_line(&mut input_b).expect("FAILED TO READ INPUT");
let b:f64 = input_b.trim().parse().expect("Please enter a valid number:");

println!("Enter a value of c:");
io::stdin().read_line(&mut input_c).expect("FAILED TO READ INPUT");
let c:f64 = input_c.trim().parse().expect("Please enter a valid number:");

let discriminant = b * b - 4.0 * a * c;

if discriminant > 0.0 {
    let root1 = (-b + discriminant.sqrt())/(2.0 * a);
    let root2 = (-b - discriminant.sqrt())/(2.0 * a);
    println!("The equation has two real roots: {} {}", root1, root2);
}else if discriminant == 0.0 {

    let root = -b / (2.0 * a);
    println!("The equation only has one real root: {}",root);
} else {
    println!("The equation has no real roots,");
}
}
