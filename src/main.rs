fn main() {
    let global = check_requirements_file_exist();
    println!("Global: {:?}", global);
}

use glob::glob;

fn check_requirements_file_exist() {
    for e in glob("*/*").expect("Failed to read glob pattern") {
        let found = e;
        // check if the file name is requirements.txt
        if found
            .unwrap()
            .display()
            .to_string()
            .contains("requirements.txt")
        {
            println!("Found a file named requirements.txt",);
        }
    }
}
