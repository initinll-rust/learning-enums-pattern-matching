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