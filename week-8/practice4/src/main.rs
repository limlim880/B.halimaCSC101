fn main() {
    let names = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
    let ages = vec![16, 17, 19, 22, 20, 21, 18, 23];

    println!("\nAge allocation:\n");

    for i in 0..ages.len() {
        println!("{} is {} years old", names[i], ages[i]);
    }
}
