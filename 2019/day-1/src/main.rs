use std::fs;

fn main() {
    let data: String = read_file("input.txt");

    let masses: Vec<i64>= data.split("\n").filter_map(|w| w.parse().ok()).collect();

    let mut planetary_fuel: i64 = 0;
    let mut total_fuel: i64 = 0;
    for mass in masses { 
        let fuel = mass_to_fuel(mass);
        planetary_fuel = planetary_fuel + fuel; 
        total_fuel = total_fuel + fuel_compensation(fuel);

    };

    println!("Fuel for planetary travel: {}\nFuel for total travel:{}", planetary_fuel, total_fuel);
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
}

fn mass_to_fuel(mass: i64) -> i64{
   (mass/3) - 2
}

fn fuel_compensation(fuel: i64) -> i64{
    if fuel < 1 {
        return 0
    };

    fuel + fuel_compensation (
        mass_to_fuel(fuel)
    )

}
