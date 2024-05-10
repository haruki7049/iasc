use clap::Parser;

fn main() {
    let args = Args::parse();
}

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(long, default_value_t = String::from(""))]
    ip_address: String,

    #[arg(long, default_value_t = String::from(""))]
    subnet_mask: String,

    #[arg(long, default_value_t = String::from(""))]
    prefix_length: String,
}

