enum IpAddrKind {
    V4,
    V6,
}

struct Ipv4Addr {
    address: (u8, u8, u8, u8)
}

struct Ipv6Addr {
    address: String
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let v4_addr = Ipv4Addr { address: (127, 0, 0, 1) };
    let home = IpAddr::V4(v4_addr);

    let v6_addr = Ipv6Addr { address: String::from("::1") };
    let loopback = IpAddr::V6(v6_addr);

//    println!("home: {:#?}", home);
}
