/*[easy] challenge #2
 * Hello, coders! An important part of programming is being able to apply your programs, so your
 * challenge for today is to create a calculator application that has use in your life. It might
 * be an interest calculator, or it might be something that you can use in the classroom. For
 * example, if you were in physics class, you might want to make a F = M * A calc.
 *
 * EXTRA CREDIT: make the calculator have multiple functions! Not only should it be able to
 * calculate F = M * A, but also A = F/M, and M = F/A!
 * */

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
