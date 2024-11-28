fn main() {
    let fullname = "Pan-Atlantic University ";
    println!("Name: {}", fullname);
    println!();
    println!("Before trim ");
    println!("length is {}", fullname.len());
    println!("After trim");
    println!("length is {}", fullname.trim().len());
}
