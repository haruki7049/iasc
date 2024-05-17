use clap::Parser;
use iasc::{prefix_to_subnet, subnet_to_prefix, Args, ConversionType, PrefixLength, SubnetMask};

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
