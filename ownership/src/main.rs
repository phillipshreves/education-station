fn main() {
    let string1 = String::from("hello");

    let length = calculate_length(&string1);

    println!("The length of '{}' is {}.", string1, length);
}

fn calculate_length(string: &String) -> usize {
    string.len()
} 
