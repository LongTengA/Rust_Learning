#[derive(Debug)]
#[allow(dead_code)]
enum IpAddressKind {
    Ip4,
    Ip6,
}
#[derive(Debug)]
#[allow(dead_code)]
struct IpAddress {
    kind: IpAddressKind,
    addr: String,
}

fn main() {
    let client = IpAddress {
        kind: IpAddressKind::Ip4,
        addr: String::from("170.192.0.0"),
    };

    println!("{:#?}", client);

    
}
