use glob::glob;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let file = check_requirements_exist_file();
    println!("requirements: {:?}", file);
    print_type_of(&file);
    read_requirements_file(file);

    let folder = check_requirements_exist_folder();
    println!("requirements: {:?}", folder);
}

fn check_requirements_exist_file() -> Vec<String> {
    let mut paths = Vec::new();
    for e in glob("*/*").expect("Failed to search for requirements file and folder") {
        let found = e.unwrap();
        if found.display().to_string().contains("requirements.txt") {
            println!("Found a file named requirements.txt, {}", found.display());
            paths.push(found.display().to_string());
        }
    }
    paths
}

fn check_requirements_exist_folder() -> Vec<String> {
    let mut paths = Vec::new();
    for e in glob("*/*").expect("Failed to search for requirements file and folder") {
        let found = e.unwrap();
        if found.display().to_string().contains("requirements") && found.is_dir() {
            println!("Found a folder named requirements, {}", found.display());
            paths.push(found.display().to_string());
        }
    }
    paths
}

fn read_requirements_file(paths: Vec<String>) -> () {
    for path in paths {
        let file = File::open(path).expect("Unable to open file");
        let reader = BufReader::new(file);
        for line in reader.lines().skip_while(|l| {
            l.as_ref().unwrap().is_empty()
                || l.as_ref().unwrap().starts_with("#")
                || l.as_ref().unwrap().starts_with("//")
            // TODO: Remove the empty lines at the end of the file
        }) {
            println!("Line: {}", line.unwrap());
        }
    }
}
