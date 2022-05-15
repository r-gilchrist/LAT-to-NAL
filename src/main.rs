use std::io;

fn main() {
    
    // Include the conversion table as text
    let data = include_str!("NALLAT18.csv");
    let intro_text = include_str!("intro_text.txt");

    println!("{}", intro_text);

    // Infinite loop so users can repeatedly use the program
    loop {

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

        // Initial lowest distance
        let mut lowest_distance: f64 = 180.0;
        let mut conversion: f64 = -999.0;

        // Loop over each line in the data and parse values
        for line in data.split("\n") {

            // Get vector of comman-separated values
            let values: Vec<&str> = line.split(",").collect();

            // Check to ensure a valid line is being read
            if values.len() == 3 {

                // Get the latitude and longitude
                let lat_test: f64 = values[0].trim().parse().unwrap();
                let lon_test: f64 = values[1].trim().parse().unwrap();

                // Calculate difference in longitude and latitude
                let diff_lat: f64 = &lat_test - &lat;
                let diff_lon: f64 = &lon_test - &lon;

                // Calculate overall distance away
                let distance = &diff_lat * &diff_lat + &diff_lon * &diff_lon;
                let distance = distance.powf(0.5);

                // Update the conversion factor if the lowest distance beats
                // the previous best value
                if distance < lowest_distance {
                    conversion = values[2].trim().parse().unwrap();
                    lowest_distance = distance;
                }
            }
        }

        // Print results to screen
        println!("Conversion from NAL to LAT is {}m", &conversion);
        println!("(nearest grid point was {:.4}° away)", &lowest_distance);

    }

    
}
