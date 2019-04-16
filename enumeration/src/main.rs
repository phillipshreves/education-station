fn main() {
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
}

