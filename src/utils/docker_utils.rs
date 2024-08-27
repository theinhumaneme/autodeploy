use crate::{ComposeConfiguation, Container};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn generate_compose(
    repo_directory: String,
    compose_directory: String,
    slug: String,
    container_config: Container,
) -> String {
    println!("Generating Compose");
    let mut services: HashMap<String, Container> = HashMap::new();
    let build_context = format!(".{}/{}", repo_directory, slug).to_owned();
    let mut container: Container = container_config.clone();
    container.build.context = build_context;
    services.insert("app".to_string(), container);
    let compose = ComposeConfiguation { services };
    let yaml = serde_yaml::to_string(&compose).unwrap();
    if !Path::new(&compose_directory).exists() {
        // Create the folder if it doesn't exist
        create_dir_all(format!("./{}", compose_directory.clone())).unwrap();
        println!("Directory created: {}", compose_directory);
    } else {
        println!("Directory already exists: {}", compose_directory);
    }
    let base_path = format!("{}/{}.yaml", compose_directory, slug)
        .to_string()
        .to_owned();
    let mut file = File::create(base_path.clone()).unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    println!("Generating Compose Complete");
    base_path
}

pub fn build_compose(compose_file_path: String) {
    let command = "docker";
    let args = ["compose", "-f", compose_file_path.as_str(), "build"];
    println!("Building Application Started");
    let mut child = Command::new(command)
        .args(&args)
        .stdin(Stdio::null()) // No input needed
        .stdout(Stdio::piped()) // Capture output
        .stderr(Stdio::piped()) // Capture error output
        .spawn()
        .unwrap();
    if let Some(stdout) = child.stdout.as_mut() {
        let output = io::BufReader::new(stdout).lines();
        for line in output {
            println!("{}", line.unwrap());
        }
    }
    // Read and print the standard error output
    if let Some(stderr) = child.stderr.as_mut() {
        let error_output = io::BufReader::new(stderr).lines();
        for line in error_output {
            eprintln!("DEBUG: {}", line.unwrap());
        }
    }
    println!("Building Application Complete");
}

pub fn start_compose(compose_file_path: String) {
    let command = "docker";
    let args = ["compose", "-f", compose_file_path.as_str(), "up", "-d"];
    println!("Trying to start application");
    let mut child = Command::new(command)
        .args(&args)
        .stdin(Stdio::null()) // No input needed
        .stdout(Stdio::piped()) // Capture output
        .stderr(Stdio::piped()) // Capture error output
        .spawn()
        .unwrap();
    if let Some(stdout) = child.stdout.as_mut() {
        let output = io::BufReader::new(stdout).lines();
        for line in output {
            println!("{}", line.unwrap());
        }
    }
    // Read and print the standard error output
    if let Some(stderr) = child.stderr.as_mut() {
        let error_output = io::BufReader::new(stderr).lines();
        for line in error_output {
            eprintln!("DEBUG: {}", line.unwrap());
        }
    }
    println!("Application Started");
}
pub fn stop_compose(compose_file_path: String) {
    let command = "docker";
    let args = ["compose", "-f", compose_file_path.as_str(), "down"];
    println!("Trying to stop application");
    let mut child = Command::new(command)
        .args(&args)
        .stdin(Stdio::null()) // No input needed
        .stdout(Stdio::piped()) // Capture output
        .stderr(Stdio::piped()) // Capture error output
        .spawn()
        .unwrap();
    if let Some(stdout) = child.stdout.as_mut() {
        let output = io::BufReader::new(stdout).lines();
        for line in output {
            println!("{}", line.unwrap());
        }
    }
    // Read and print the standard error output
    if let Some(stderr) = child.stderr.as_mut() {
        let error_output = io::BufReader::new(stderr).lines();
        for line in error_output {
            eprintln!("DEBUG: {}", line.unwrap());
        }
    }
    println!("Application Stopped");
}
pub fn restart_compose(compose_file_path: String) {
    stop_compose(compose_file_path.clone());
    start_compose(compose_file_path.clone());
}