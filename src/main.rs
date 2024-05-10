use clap::Parser;

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
    prefix_length: Option<String>,
}
