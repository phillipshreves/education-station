fn main() {
    //enumeration setup
    enum IpAddressKind {
        V4,
        V6,
    }

    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddress::V4(127, 0, 0, 1);

    let loopback = IpAddress::V6(String::from("::1"));

    //Methods

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
        }

    }
    
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    //Match control flow operator
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        California,
        Texas,
    }

    enum Coin {
        Penny, 
        Nickel, 
        Dime, 
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    let value_return = value_in_cents(
        Coin::Quarter(UsState::Alaska)
    ); 
}

