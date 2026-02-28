mod error;
mod prelude;

use clap::Parser;

#[derive(Parser)]
#[command(name = "pagelens", about = "Audit web pages for contrast, SEO, accessibility, and performance")]
struct Cli {
    /// URL to analyze
    url: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(url) = cli.url {
        println!("Analyzing: {url}");
    } else {
        println!("Usage: pagelens <url>");
    }
}
