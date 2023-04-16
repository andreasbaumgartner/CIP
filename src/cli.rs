use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    command: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

pub fn run_cli() {
    let args = Cli::parse();
    println!("{:?}", args);
}
