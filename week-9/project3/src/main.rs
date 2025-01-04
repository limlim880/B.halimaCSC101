fn main() {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyeye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("Merged EFCC Dataset:\n");
    println!("S/N\tName of Commissioner\tMinistry\t\tGeopolitical Zone");

    for i in 0..commissioners.len() {
        println!(
            "{}\t{}\t{}\t{}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }
}
