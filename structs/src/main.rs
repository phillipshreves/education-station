use std::io;

fn main() {
    let mut flight_1 = Flight {
        origin_location: String::from("DFW"),
        destination_location: String::from("LAX"),
        flight_number: 1234,
    };

    println!("Please confirm destination: {}", flight_1.destination_location);
    let mut destination_confirm = String::new();
    println!("Enter destination:");
    io::stdin().read_line(&mut destination_confirm)
        .expect("Failed to get input");
    
    flight_1.destination_location = destination_confirm;
    println!("Destination confirmed: {}", flight_1.destination_location);

    let mut flight_2 = build_flight(String::from("LHR"), String::from("ORD"), 4321);
    println!("New flight: {},{},{}", flight_2.origin_location, flight_2.destination_location, flight_2.flight_number);

    let flight_3 = Flight {
        destination_location: String::from("FCA"),
        ..flight_2
    };
    println!("{}", flight_3.origin_location);
}

struct Flight {
    origin_location: String,
    destination_location: String,
    flight_number: u16,
}

fn build_flight (origin_location: String, destination_location: String, flight_number: u16) -> Flight {
    Flight{
        origin_location,
        destination_location,
        flight_number,
    }
}
