fn main() {
    another_function(5, 6);

    let five = plus_one(five());

    println!("The variable five is {}", five);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(input: i32) -> i32 {
    input + 1
}
