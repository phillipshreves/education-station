/*Challenge #4 [easy]
 * You're challenge for today is to create a random password generator!
 *
 * For extra credit, allow the user to specify the amount of passwords to generate.
 *
 * For even more extra credit, allow the user to specify the length of the strings he wants to
 * generate!
 * */


extern crate rand;
use rand::prelude::*;

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("input");
    String::from(input.trim())
}

fn main() {
    println!("Number of passwords to create:");
    let passwords_to_create = 0..get_input().parse::<u32>().unwrap_or(1);
    println!("Length of passwords:");
    let passwords_length = get_input().parse::<u32>().unwrap_or(1);

    for index in passwords_to_create {
        let mut random_string = String::from("");
        let mut counter = 0;
        loop {
            counter += 1;

            let random: u8 = thread_rng().gen_range(65, 122);
            let random = match random {
                91..=96 => thread_rng().gen_range(65, 90),
                _ => random,
            };
            let random_char = random as char;
            let random_string = random_string.push_str(&random_char.to_string());
            if counter == passwords_length {
                break random_string;
            };
        };
        println!("{}", random_string );
    };
}
