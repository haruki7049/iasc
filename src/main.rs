use clap::Parser;
use clap::ValueEnum;
use std::net::Ipv4Addr;

type SubnetMask = Ipv4Addr;

fn main() {
    let args = Args::parse();

    match args.conversion_type {
        Some(ConversionType::SubnetToPrefix) => {
            let subnet_mask: SubnetMask = args
                .subnet_mask
                .expect("No input for subnet_mask")
                .parse()
                .expect("Invalid subnet mask");
            println!("{}", subnet_to_prefix(subnet_mask).unwrap());
        }
        Some(ConversionType::PrefixToSubnet) => {
            let prefix_length: PrefixLength =
                PrefixLength::new(args.prefix_length.expect("Invalid prefix length"))
                    .expect("Invalid prefix length");
            println!("{}", prefix_to_subnet(prefix_length).unwrap());
        }
        None => {
            panic!("You should specify --conversion-type option...");
        }
    }
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

    #[arg(short, long)]
    conversion_type: Option<ConversionType>,
}

// This enum is used whether the user wants to convert subnet mask to prefix length or prefix length to subnet mask.
#[derive(Clone, Debug, ValueEnum)]
enum ConversionType {
    SubnetToPrefix,
    PrefixToSubnet,
}

impl std::str::FromStr for ConversionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "subnet-to-prefix" => Ok(ConversionType::SubnetToPrefix),
            "prefix-to-subnet" => Ok(ConversionType::PrefixToSubnet),
            _ => Err(String::from("Invalid conversion type")),
        }
    }
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
        _ => Err(String::from(
            "Cannot calcurate the SubnetMask... Perhaps, do you input invalid SubnetMask?",
        )),
    }
}

/// This function return the subnet mask from the prefix length.
pub fn prefix_to_subnet(prefix: PrefixLength) -> Result<SubnetMask, String> {
    let prefix_string: String = prefix.to_string();
    match &prefix_string[..] {
        "32" => Ok(SubnetMask::new(255, 255, 255, 255)),
        "31" => Ok(SubnetMask::new(255, 255, 255, 254)),
        "30" => Ok(SubnetMask::new(255, 255, 255, 252)),
        "29" => Ok(SubnetMask::new(255, 255, 255, 248)),
        "28" => Ok(SubnetMask::new(255, 255, 255, 240)),
        "27" => Ok(SubnetMask::new(255, 255, 255, 224)),
        "26" => Ok(SubnetMask::new(255, 255, 255, 192)),
        "25" => Ok(SubnetMask::new(255, 255, 255, 128)),
        "24" => Ok(SubnetMask::new(255, 255, 255, 0)),
        "23" => Ok(SubnetMask::new(255, 255, 254, 0)),
        "22" => Ok(SubnetMask::new(255, 255, 252, 0)),
        "21" => Ok(SubnetMask::new(255, 255, 248, 0)),
        "20" => Ok(SubnetMask::new(255, 255, 240, 0)),
        "19" => Ok(SubnetMask::new(255, 255, 224, 0)),
        "18" => Ok(SubnetMask::new(255, 255, 192, 0)),
        "17" => Ok(SubnetMask::new(255, 255, 128, 0)),
        "16" => Ok(SubnetMask::new(255, 255, 0, 0)),
        "15" => Ok(SubnetMask::new(255, 254, 0, 0)),
        "14" => Ok(SubnetMask::new(255, 252, 0, 0)),
        "13" => Ok(SubnetMask::new(255, 248, 0, 0)),
        "12" => Ok(SubnetMask::new(255, 240, 0, 0)),
        "11" => Ok(SubnetMask::new(255, 224, 0, 0)),
        "10" => Ok(SubnetMask::new(255, 192, 0, 0)),
        "9" => Ok(SubnetMask::new(255, 128, 0, 0)),
        "8" => Ok(SubnetMask::new(255, 0, 0, 0)),
        "7" => Ok(SubnetMask::new(254, 0, 0, 0)),
        "6" => Ok(SubnetMask::new(252, 0, 0, 0)),
        "5" => Ok(SubnetMask::new(248, 0, 0, 0)),
        "4" => Ok(SubnetMask::new(240, 0, 0, 0)),
        "3" => Ok(SubnetMask::new(224, 0, 0, 0)),
        "2" => Ok(SubnetMask::new(192, 0, 0, 0)),
        "1" => Ok(SubnetMask::new(128, 0, 0, 0)),
        _ => Err(String::from(
            "Cannot calcurate the PrefixLength... Perhaps, do you input invalid PrefixLength?",
        )),
    }
}

#[derive(Debug)]
pub struct PrefixLength {
    pub length: u8,
}

impl std::fmt::Display for PrefixLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.length)
    }
}

impl PrefixLength {
    pub fn new(length: usize) -> Result<Self, String> {
        match length {
            0 => Err(String::from("Prefix length must not be 0")),
            1..=32 => Ok(PrefixLength {
                length: length as u8,
            }),
            33.. => Err(String::from("Prefix length must not be greater than 32")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{prefix_to_subnet, subnet_to_prefix, PrefixLength, SubnetMask};

    #[test]
    fn test_prefix_to_subnet() {
        let prefix: PrefixLength = PrefixLength::new(24).unwrap();

        assert_eq!(
            prefix_to_subnet(prefix).unwrap().to_string(),
            "255.255.255.0"
        );
    }

    #[test]
    #[should_panic(expected = "Prefix length must not be 33 ~ 128")]
    fn test_invalid_prefix() {
        let _invalid_prefix: PrefixLength = PrefixLength::new(33).unwrap();
    }

    #[test]
    #[should_panic(expected = "Prefix length must not be 0")]
    fn test_zero_prefix() {
        let _zero_prefix: PrefixLength = PrefixLength::new(0).unwrap();
    }

    #[test]
    fn test_subnet_to_prefix() {
        const SUBNET: SubnetMask = SubnetMask::new(255, 255, 255, 0);
        const INVALID_SUBNET: SubnetMask = SubnetMask::new(255, 255, 255, 123);

        assert_eq!(subnet_to_prefix(SUBNET).unwrap().to_string(), "24");
        assert_eq!(
            subnet_to_prefix(INVALID_SUBNET).unwrap_err(),
            "Cannot calcurate the SubnetMask... Perhaps, do you input invalid SubnetMask?"
        );
    }
}
