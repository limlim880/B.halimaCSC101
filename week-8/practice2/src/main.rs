fn main() {
    let v = vec!['C', 'O', 'N', 'P', 'U', 'T', 'E', 'R'];

    let mut input1 = String::new();

    println!("Enter an index value between (0-7):");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index: usize = input1.trim().parse().expect("Invalid input");

    if index < v.len() {
        let ch: char = v[index];
        println!("{} is the character for index [{}]\n", ch, index);
    } else {
        println!("Index out of range. Please enter a value between 0 and 7.");
    }
}
