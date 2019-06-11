
fn get_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(string) => input,
        Err(error) => String::new()
    }
}

fn encrypt(message: &String) -> String {
    let mut encrypted = String::new();
    for char in message.chars() {
        let char_number = char as u8;
        let char_encryption = if char_number + 13 > 122 {
            char_number + 13 - 57
        } else {
            char_number + 13
        };
        let char_new = char_encryption as char;
        match char_encryption {
            65..=122 => encrypted.push_str(&char_new.to_string()),
            _ => ()
        }
    }
    encrypted
}

fn main() {
    println!("Message to encrypt:");
    let message = get_input();
    let encrypted_message = encrypt(&message);
    println!("Encrypted message: {}", encrypted_message);
}
