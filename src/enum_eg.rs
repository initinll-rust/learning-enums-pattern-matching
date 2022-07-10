#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub struct IpAddr {
    pub kind: IpAddrKind,
    pub address: String,
}