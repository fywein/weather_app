use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// City name
    #[arg(short, long)]
    pub city: String,
}
