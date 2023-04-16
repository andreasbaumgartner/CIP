use std::process::Command;

struct Cli {
    pub command: String,
    pub args: Vec<String>,
}

fn cli() {
    let command = Command::new("pip")
        .arg("install")
        .arg("-r")
        .arg("requirements.txt")
        .output()
        .expect("Failed to execute pip install -r requirements.txt");
    println!("stdout: {}", String::from_utf8_lossy(&command.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&command.stderr));
}
