enum IpAddrKind{
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddress {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("ipv4"),
        IpAddrKind::V6 => println!("ipv6"),
    }
}

fn route_new(addr: IpAddress) {
    match addr {
        IpAddress::V4(v4) => println!("ipv4: {}", v4),
        IpAddress::V6(v6) => println!("ipv6: {}", v6),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let loopback4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback4 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from(":1"),
    };

    let v4 = IpAddress::V4(String::from("127.0.0.1"));
    let v6 = IpAddress::V6(String::from(":1"));
    route_new(v4);
    route_new(v6);

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let ret = plus_one(some_number);

    match ret {
        Some(6) => println!("number is 6"),
        _ => ()
    }

    if let Some(6) = ret {
        println!("number is 6");
    } else {
        println!("other values");
    }
}
