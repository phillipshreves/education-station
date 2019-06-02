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
