use clap::Parser;
use std::net::Ipv4Addr;

type SubnetMask = Ipv4Addr;

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

pub fn subnet_to_prefix(subnet: SubnetMask) -> Result<PrefixLength, String> {
    let subnet_string: String = subnet.to_string();
    match &subnet_string[..] {
        "255.255.255.255" => Ok(PrefixLength::new(32)?),
        "255.255.255.254" => Ok(PrefixLength::new(31)?),
        "255.255.255.252" => Ok(PrefixLength::new(30)?),
        "255.255.255.248" => Ok(PrefixLength::new(29)?),
        "255.255.255.240" => Ok(PrefixLength::new(28)?),
        "255.255.255.224" => Ok(PrefixLength::new(27)?),
        "255.255.255.192" => Ok(PrefixLength::new(26)?),
        "255.255.255.128" => Ok(PrefixLength::new(25)?),
        "255.255.255.0" => Ok(PrefixLength::new(24)?),
        "255.255.254.0" => Ok(PrefixLength::new(23)?),
        "255.255.252.0" => Ok(PrefixLength::new(22)?),
        "255.255.248.0" => Ok(PrefixLength::new(21)?),
        "255.255.240.0" => Ok(PrefixLength::new(20)?),
        "255.255.224.0" => Ok(PrefixLength::new(19)?),
        "255.255.192.0" => Ok(PrefixLength::new(18)?),
        "255.255.128.0" => Ok(PrefixLength::new(17)?),
        "255.255.0.0" => Ok(PrefixLength::new(16)?),
        "255.254.0.0" => Ok(PrefixLength::new(15)?),
        "255.252.0.0" => Ok(PrefixLength::new(14)?),
        "255.248.0.0" => Ok(PrefixLength::new(13)?),
        "255.240.0.0" => Ok(PrefixLength::new(12)?),
        "255.224.0.0" => Ok(PrefixLength::new(11)?),
        "255.192.0.0" => Ok(PrefixLength::new(10)?),
        "255.128.0.0" => Ok(PrefixLength::new(9)?),
        "255.0.0.0" => Ok(PrefixLength::new(8)?),
        "254.0.0.0" => Ok(PrefixLength::new(7)?),
        "252.0.0.0" => Ok(PrefixLength::new(6)?),
        "248.0.0.0" => Ok(PrefixLength::new(5)?),
        "240.0.0.0" => Ok(PrefixLength::new(4)?),
        "224.0.0.0" => Ok(PrefixLength::new(3)?),
        "192.0.0.0" => Ok(PrefixLength::new(2)?),
        "128.0.0.0" => Ok(PrefixLength::new(1)?),
        _ => Err(String::from("Cannot calcurate subnetMask")),
    }
}

pub struct PrefixLength {
    pub length: u8
}

impl std::fmt::Display for PrefixLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.length)
    }
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
    use crate::{
        calc_net_addr,
        subnet_to_prefix,
        PrefixLength,
        SubnetMask,
    };
    use std::net::Ipv4Addr;

    #[test]
    fn test_calc_net_addr() {
        let ip: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 1);
        let prefix: PrefixLength = PrefixLength::new(24).unwrap();

        assert_eq!(calc_net_addr(ip, prefix).unwrap().to_string(), "192.168.0.0");
    }

    #[test]
    fn test_calc_subnet() {
        let prefix: SubnetMask = SubnetMask::new(255, 255, 255, 0);

        assert_eq!(subnet_to_prefix(prefix).unwrap().to_string(), "24");
    }
}
