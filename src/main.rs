pub mod enum_eg;

use enum_eg::{IpAddrKind1, IpAddr1, IpAddr2};

fn main() {
    // combination of struct & enum
    let home1 = IpAddr1 {
        kind: IpAddrKind1::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback1 = IpAddr1 {
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };

    println!("{:?} {:?}", home1, loopback1);

    // without struct & using only enum
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("{:?} {:?}", home2, loopback2);
}


