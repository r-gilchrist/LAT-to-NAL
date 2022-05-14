use std::io;

fn main() {
    
    // Include the conversion table as text
    let data = include_str!("NALLAT18.csv");

    // User input variables
    let mut lat = String::new();
    let mut lon = String::new();

    // Read latitude from command line
    println!("Enter latitude (decimal degrees):");
    io::stdin()
        .read_line(&mut lat)
        .expect("Could not read line");

    // Read longitude from command line
    println!("Enter longitude (decimal degrees):");
    io::stdin()
        .read_line(&mut lon)
        .expect("Could not read line");

    // Latitude and longitude should be floats
    let lat: f64 = lat.trim().parse().expect("Please enter a number.");
    let lon: f64 = lon.trim().parse().expect("Please enter a number.");

    // Summarise the user input
    println!("You entered {}°N, {}°E", &lat, &lon);

}
