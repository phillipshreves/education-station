use std::fs;

fn main() {
    let data: String = read_file("input.txt");

    let masses: Vec<i64>= data.split("\n").filter_map(|w| w.parse().ok()).collect();

    let mut total_fuel: i64 = 0;
    for mass in masses { 
       total_fuel = total_fuel + mass_to_fuel(mass); 
    };

    println!("{}", total_fuel);
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
}

fn mass_to_fuel(mass: i64) -> i64{
   (mass/3) - 2
}
