fn main() {
    let file = check_requirements_file_exist();
    println!("Global: {:?}", file);

    let folder = does_folder_foo_exist_in_current_directory();
    println!("Global: {:?}", folder);
}

use glob::glob;

fn check_requirements_file_exist() {
    for e in glob("*/*").expect("Failed to search for requirements file") {
        let found = e.unwrap();
        if found.display().to_string().contains("requirements.txt") {
            println!("Found a file named requirements.txt, {}", found.display());
        }
    }
}

fn does_folder_foo_exist_in_current_directory() {
    for e in glob("*/*").expect("Failed to search for requirements folder") {
        let found = e.unwrap();
        println!("Found: {:?}", found);
        if found.display().to_string().contains("requirements") && found.is_dir() {
            println!("Found a folder named requirements, {}", found.display());
        }
    }
}
