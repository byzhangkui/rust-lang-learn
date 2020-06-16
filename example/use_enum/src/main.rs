enum IpAddrKind{
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    if ip_kind == IpAddrKind::V4 {
        println!("ipv4");
    } else if ip_kind == IpAddrKind::V6 {
        println!("ipv6");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");
}
