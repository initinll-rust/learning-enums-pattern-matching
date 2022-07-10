#[derive(Debug)]
pub enum IpAddrKind1 {
    V4,
    V6,
}
#[derive(Debug)]
pub struct IpAddr1 {
    pub kind: IpAddrKind1,
    pub address: String,
}

#[derive(Debug)]
pub enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
pub enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        // method body would be defined here
    }
}
