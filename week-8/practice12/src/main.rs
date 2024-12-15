fn main() {
    let mut colors = ["Red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    // Correct variable name for the mutable slice
    let slice_colors = &mut colors[1..3];
    println!("First slice = {:?}", slice_colors);

    // Modify an element in the slice
    slice_colors[1] = "purple";

    println!("Changed slice = {:?}", slice_colors);
    println!("Updated array = {:?}", colors);
}
