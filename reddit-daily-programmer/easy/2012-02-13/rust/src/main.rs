/*Challenge #5 [easy]
 * Your challenge for today is to create a program which is password protected, and wont open
 * unless the correct user and password is given.
 *
 * For extra credit, have the user and password in a seperate .txt file.
 *
 * for even more extra credit, break into your own program :)
*/


// look at https://docs.rs/csv/1.1.1/csv/tutorial/index.html

use std::io;

fn get_csv_values() -> (){
    let mut reader = csv::Reader::from_reader(io::stdin());
    
    for value in reader.records() {
        let record = value.expect("a CSV record");
        println!("{:?}", record)
    }

    ()
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("input");
    String::from(input.trim())
}

fn get_value() -> String {
   String::from("Phil") 
}

fn verify_user(user: String) -> bool {
    if get_value() == user {
        return true
    } else {
        return false
    }
}

fn verify_password(password: String) -> bool {
    if get_value() == password {
        return true
    } else {
        return false
    }
}

fn main() {
    let csv = get_csv_values();
    println!("Please Enter Username:");
    let username = get_input();
    println!("Please Enter Password:");
    let password = get_input();

    if verify_user(username) {
        if verify_password(password) {
            println!("Success");
        }
    }
}
