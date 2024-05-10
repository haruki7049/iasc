use clap::Parser;
use std::net::Ipv4Addr;

fn main() {
    let args = Args::parse();
}

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(long)]
    ip_address: Option<String>,

    #[arg(long)]
    subnet_mask: Option<String>,

    #[arg(long)]
    prefix_length: Option<usize>,
}

pub fn calc_net_addr(ip: Ipv4Addr, prefix: PrefixLength) -> Result<Ipv4Addr, String> {
    todo!();
}

pub struct PrefixLength {
    pub length: u8
}

impl PrefixLength {
    pub fn new(length: u8) -> Result<Self, String> {
        match length {
            0 => Err(String::from("Prefix length must not be 0")),
            1..=32 => Ok(PrefixLength {
                length
            }),
            33..=u8::MAX => Err(String::from("Prefix length must not be 33 ~ 128")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::calc_net_addr;
    use std::net::Ipv4Addr;
    use crate::PrefixLength;

    #[test]
    fn test_calc_net_addr() {
        let ip: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 1);
        let prefix: PrefixLength = PrefixLength::new(24).unwrap();

        assert_eq!(calc_net_addr(ip, prefix).unwrap().to_string(), "192.168.0.0");
    }
}
