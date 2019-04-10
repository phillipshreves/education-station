fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hellor");

    let s3 = takes_and_gives_back(s2);

    println!("{}, {}", s1,  s3);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}
