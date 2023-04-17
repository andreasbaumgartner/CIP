use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, author, about)]

pub struct Commmands {
    /// Update the package list
    pub update: String,
    /// Search for a package
    pub search: String,
    /// Install a package
    pub install: String,
    /// Remove a package
    pub remove: String,
    /// List installed packages
    pub list: String,
    /// Show help about a package
    pub help: String,
    /// Show version about a package
    pub version: String,
    /// Show information about a package
    pub info: String,
    /// clean up the package cache and remove unused packages
    pub clean: String,
    /// Show the settings
    pub settings: String,
}

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
