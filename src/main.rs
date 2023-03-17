use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    url: String,
}


fn main() {
    let cli = Cli::parse();
    open_tab::open_tab(&cli.url);
}