use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("nigerian_breweries.txt")
        .expect("Could not create file.");

    file.write_all(b"Nigerian Breweries High-Quality Categories of Drinks:\n")
        .expect("Write failed.");
    file.write_all(b"\nLager:\n- 33 Export\n- Desperados\n- Goldberg\n- Gulder\n- Heineken\n- Star\n")
        .expect("Write failed.");
    file.write_all(b"\nStout:\n- Legend\n- Turbo King\n- Williams\n")
        .expect("Write failed.");
    file.write_all(b"\nNon-Alcoholic:\n- Maltina\n- Amstel Malta\n- Malta Gold\n- Fayrouz\n")
        .expect("Write failed.");

    println!("File written successfully!");
}
