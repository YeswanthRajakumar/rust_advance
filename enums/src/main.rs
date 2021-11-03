#[derive(Debug)]
enum IpAddressType {
    V4(u8,u8,u8,u8),
    V6(String),
}



fn route(ip_type: IpAddressType) {}

fn main() {
    // Defining ENUM
    let four = IpAddressType::V4;
    let six = IpAddressType::V6;
    // println!("{:#?}", four);

    // let my_ip = IpAddress {
    //     address: String::from("127.0.0.1"),
    //     version: IpAddressType::V4,
    // };

    // Associated Value
    let four = IpAddressType::V4(127,0,0,1);
    let six = IpAddressType::V6(String::from("127.0.0.1"));

    println!("{:#?}", my_ip);
}
