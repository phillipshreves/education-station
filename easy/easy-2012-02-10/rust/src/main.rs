/*[easy] challenge #1
 * create a program that will ask the users name, age, and reddit username. have it tell them the
 * information back, in the format:
 *
 * your name is (blank), you are (blank) years old, and your username is (blank)
 * for extra credit, have the program log this information in a file to be accessed later.
 * */

use std::io;
use std::io::prelude::*; 

fn read_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input");

    String::from(input.trim())
}

fn main() {
    println!("Please enter the following -");
    print!("Name: ");
    io::stdout().flush().unwrap();
    let name = read_input();
    print!("Age: ");
    io::stdout().flush().unwrap();
    let age = read_input();
    print!("Username: ");
    io::stdout().flush().unwrap();
    let username = read_input();

    println!("your name is {name}, you are {age} years old, and your username is {username}",
             name=name,
             username=username,
             age=age);
}
