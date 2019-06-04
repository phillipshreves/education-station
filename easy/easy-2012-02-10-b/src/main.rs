
fn read_input() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("input");

    String::from(input.trim())
}

fn main() {
    // collect input data from user
    println!("Force=Mass*Acceleration Calculator\nForce:");
    let force = read_input();
    println!("Mass:");
    let mass = read_input();
    println!("Acceleration:");
    let acceleration = read_input();

    // unwrap the input answers, and parse into a number
    // if any answer is not a number, return a 1 to use in calculation
    let force: f64 = force.parse::<f64>().unwrap_or(1.0);
    let mass: f64 = mass.parse::<f64>().unwrap_or(1.0);
    let acceleration: f64 = acceleration.parse::<f64>().unwrap_or(1.0);
    println!("{}={}*{}", force, mass, acceleration );

    let equals = (mass*acceleration)/force;
    println!("{}", equals);
}
