use std::io;

fn main() {

    let minimum_lat: f64 = 51.3;
    let maximum_lat: f64 = 55.8;
    let minimum_lon: f64 = 2.5;
    let maximum_lon: f64 = 7.5;

    let data = include_str!("NALLAT18.csv");

    let introduction_text = include_str!("intro_text.txt");
    println!("{}", introduction_text);

    loop {

        println!("\n--------Next-Iteration--------\n");

        // User input of latitude
        let mut lat = String::new();
        println!("Enter latitude (range: 51.3° - 55.8°N):");
        io::stdin()
            .read_line(&mut lat)
            .expect("Could not read line");

        // Latitude should be a float
        let lat: f64 = match lat.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid latitude '{}'! Please try again!", &lat.trim());
                continue
            }
        };

        // Check latitude is within bounds
        if &lat < &minimum_lat || &lat > &maximum_lat {
            println!("Latitude is out of range! Please try again");
            continue
        };

        // User input of longitude
        let mut lon = String::new();
        println!("Enter longitude (range: 2.5°E - 7.5°E):");
        io::stdin()
            .read_line(&mut lon)
            .expect("Could not read line");

        // Longitude should be a float
        let lon: f64 = match lon.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid longitude '{}'! Please try again!", &lon.trim());
                continue
            }
        };

        // Check longitude is within bounds
        if &lon < &minimum_lon || &lon > &maximum_lon {
            println!("Longitude is out of range! Please try again");
            continue
        };

        println!("\nYou entered {}°N, {}°E\n", &lat, &lon);

        // Furthest possible distance away
        let mut lowest_distance: f64 = 180.0;
        let mut conversion_factor: f64 = -999.0;

        for line in data.split("\n") {

            // values are:
            // [0] -> latitude
            // [1] -> longitude
            // [2] -> conversion factor
            let values: Vec<&str> = line.split(",").collect();

            if values.len() == 3 {

                let lat_test: f64 = values[0].trim().parse().unwrap();
                let lon_test: f64 = values[1].trim().parse().unwrap();

                let diff_lat: f64 = &lat_test - &lat;
                let diff_lon: f64 = &lon_test - &lon;

                let current_distance = &diff_lat * &diff_lat + &diff_lon * &diff_lon;
                let current_distance = current_distance.powf(0.5);

                // Check if new location is nearer than previous
                if current_distance < lowest_distance {
                    conversion_factor = values[2].trim().parse().unwrap();
                    lowest_distance = current_distance;
                }
            }
        }

        println!("To convert from NAL to LAT, +{:.3}m", &conversion_factor);
        println!("To convert from LAT to NAL, -{:.3}m\n", &conversion_factor);
        println!("NOTE - the nearest grid point was {:.3}° away", &lowest_distance);

    }
}
