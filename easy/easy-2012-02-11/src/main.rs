
fn get_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(string) => input,
        Err(error) => String::from("")
    }
}

fn encrypt(message: String) -> String {
    for character in message.chars() {
        let character_new = std::char::from_u32(character as u32 - 13);
        let mut encrypted: String = "";
        //encrypted.push(character_new);
        match
    }
}

fn main() {
    let message = get_input();
    if message.is_empty() {
        println!("Cannot encrypt empty message");
    } else {
        let encrypted_message = encrypt(message);
        println!("Encrypted message: {}", encrypted_message);
    }
}
