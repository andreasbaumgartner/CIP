use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct SearchPackages {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

pub fn search_for_packages() {
    let args = SearchPackages::parse();
    let contents =
        std::fs::read_to_string(args.path).expect("Something went wrong reading the file");

    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
