pub mod enum_eg;
pub mod match_eg;

use enum_eg::*;
use match_eg::*;

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

    // storing different data type
    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));
    println!("{:?} {:?}", home3, loopback3);

    //  Message enum whose variants each store different amounts and types of values
    let m = Message::Write(String::from("hello"));
    m.call();

    // match patterns
    let _ = Coin::Quarter(UsState::Alabama);
    let five = Some(5);
    let _ = plus_one(five);
    let _ = plus_one(None);

    // Catch-all Patterns and the _ Placeholder
    let dice_roll1 = 9;
    match dice_roll1 {
        3 => (),
        7 => (),
        other => (),
    }

    let dice_roll2 = 9;
    match dice_roll2 {
        3 => (),
        7 => (),
        _ => (),
    }
}


