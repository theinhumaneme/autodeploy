use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io,
    io::{BufRead, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::{check_file, ComposeConfiguation, Container};

pub fn generate_compose(
    repo_directory: &str,
    compose_directory: &str,
    slug: &str,
    container_config: &Container,
) -> String {
    let mut services: HashMap<String, Container> = HashMap::new();
    let build_context: String = format!(".{}/{}", repo_directory, slug).to_owned();
    let mut container: Container = container_config.clone();
    container.build.context = build_context;
    services.insert("app".to_string(), container);
    let compose = ComposeConfiguation { services };
    let yaml = serde_yaml::to_string(&compose).unwrap();
    if !Path::new(&compose_directory).exists() {
        // Create the folder if it doesn't exist
        create_dir_all(format!("./{}", compose_directory)).unwrap();
        println!("Directory created:{}", compose_directory);
    } else {
        println!("Directory already exists: {}", compose_directory);
    }
    let base_path = format!("{}/{}.yaml", compose_directory, slug).to_string();
    let mut file = File::create(&base_path).unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    println!("Generating Compose Complete");
    base_path
}
pub fn execute_command(command: &str, args: Vec<&str>) {
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
}

pub fn build_compose(compose_file_path: &str) {
    let command = "docker";
    let args = ["compose", "-f", compose_file_path, "build"];
    if check_file(&compose_file_path) {
        execute_command(command, args.to_vec());
    }
}

pub fn start_compose(compose_file_path: &str, project: &str) {
    let command = "docker";
    let args = [
        "compose",
        "-f",
        compose_file_path,
        "-p",
        project,
        "up",
        "-d",
    ];
    if check_file(&compose_file_path) {
        execute_command(command, args.to_vec());
    }
}
pub fn stop_compose(compose_file_path: &str, project: &str) {
    let command = "docker";
    let args = ["compose", "-f", compose_file_path, "-p", project, "down"];
    if check_file(&compose_file_path) {
        execute_command(command, args.to_vec());
    }
}
pub fn restart_compose(compose_file_path: &str, project: &str) {
    stop_compose(&compose_file_path, &project);
    start_compose(&compose_file_path, &project);
}
