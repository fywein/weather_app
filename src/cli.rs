use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// City name
    #[arg(short = 'c', long, default_value = "Seoul")]
    pub city: String,

    /// Country code (e.g., KR)
    #[arg(short = 'o', long, default_value = "KR")]
    pub country: String,
}
