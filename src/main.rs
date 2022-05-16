use std::io;

fn main() {
    
    // Include the conversion table as text
    let data = include_str!("NALLAT18.csv");
    let intro_text = include_str!("intro_text.txt");

    // Introduction text to screen
    println!("{}", intro_text);

    // Infinite loop so users can repeatedly use the program
    loop {

        // Separator line
        println!("\n--------Next-Iteration--------\n");

        // User input variables
        let mut lat = String::new();
        let mut lon = String::new();

        // Read latitude from command line
        println!("Enter latitude (range: 51.3° - 55.8°N):");
        io::stdin()
            .read_line(&mut lat)
            .expect("Could not read line");

        // Latitude should be a float. Restart program if not!
        let lat: f64 = match lat.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid latitude '{}'! Please try again!", &lat.trim());
                continue
            }
        };

        // Latitude should be within the specified range
        if lat < 51.3 || lat > 55.8 {
            println!("Latitude is out of range! Please try again");
            continue
        };

        // Read longitude from command line
        println!("Enter longitude (range: 2.5°E 7.5°E):");
        io::stdin()
            .read_line(&mut lon)
            .expect("Could not read line");

        // Longitude should be a float. Restart program if not!
        let lon: f64 = match lon.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid longitude '{}'! Please try again!", &lon.trim());
                continue
            }
        };

        // Longitude should be within the specified range
        if lon < 2.5 || lon > 7.5 {
            println!("Longitude is out of range! Please try again");
            continue
        };

        // Summarise the user input
        println!("\nYou entered {}°N, {}°E\n", &lat, &lon);

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
        println!("To convert from NAL to LAT, +{:.3}m", &conversion);
        println!("To convert from LAT to NAL, -{:.3}m\n", &conversion);

        // Comment about distance to nearest grid point
        println!("NOTE - the nearest grid point was {:.3}° away", &lowest_distance);

    }
}
