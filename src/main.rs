use dotenv::dotenv;
use inquire::InquireError;
use inquire::Select;
use objects::structs::Service;
use std::fs;
use std::path::Path;
use std::process::exit;
use std::slice::Iter;
use text_to_ascii_art::to_art;
use toml;
use utils::git_utils::check_repository;
use utils::git_utils::prompt_clone_repository;

mod objects;
mod utils;
use objects::structs::GlobalConfiguration;
use objects::structs::ProjectConfiguation;
// USER FLOW
// prompt for operation, deploy restart or stop
// prompt for the appropriate project
// prompt for branch in the project if operation is deployment
// else use the docker bindings or the os to operate using the files generated
//

fn init() -> String {
    let global_configuration_file = "global.toml";
    let global_configuration = match fs::read_to_string(global_configuration_file) {
        Ok(c) => {
            // println!("Successfully read global configuration");
            c
        }
        Err(_) => {
            eprintln!("Could not read file `{}`", global_configuration_file);
            exit(1);
        }
    };
    let config: GlobalConfiguration = match toml::from_str(&global_configuration) {
        Ok(d) => {
            // println!("Successfully loaded global configuration");
            d
        }
        Err(_) => {
            eprintln!("Unable to load data from `{}`", global_configuration_file);
            exit(1);
        }
    };
    let print_banner: bool = config.print_banner;
    if print_banner {
        match to_art("AUTO DEPLOY".to_string(), "default", 0, 0, 0) {
            Ok(string) => println!(
                "{}\n\nConfigurable custom wrapper over for quick and hassle free deployments",
                string
            ),
            Err(err) => println!("Error: {}", err),
        }
    } else {
        println!("Auto Deploy - Internal Tooling")
    }
    print!("\nAuthor: Kalyan Mudumby (@theinhumaneme / theinhumaneme@gmail.com)");
    if config.organization.is_some() {
        print!("\nOrganization: {}", config.organization.unwrap());
    }
    if config.client.is_some() {
        print!("\nClient: {}", config.client.unwrap());
    }
    println!("\n"); // standard gutter
    return config.configuration_file;
}
fn main() {
    dotenv().ok();
    let git_username = std::env::var("GIT_USERNAME").expect("GIT_USERNAME must be set.");
    let git_password: String = std::env::var("GIT_PASSWORD").expect("GIT_PASSWORD must be set.");
    let configuration_file = match fs::read_to_string(init()) {
        Ok(c) => {
            // println!("Successfully read project configuration file");
            c
        }
        Err(_) => {
            // eprintln!("Could not read file project configuration file");
            exit(1);
        }
    };
    let config: ProjectConfiguation = match toml::from_str(&configuration_file) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Invalid Project Configuration Detected");
            exit(1);
        }
    };
    let operations: Vec<&str> = vec![
        "Deploy Application",
        "Restart Application",
        "Stop Application",
    ];
    let operation_choice: Result<&str, InquireError> =
        Select::new("What would you like to do?", operations).prompt();
    let project_iterator: Iter<Service> = config.service.iter();
    let projects = project_iterator
        .clone()
        .map(|service| service.name.as_str())
        .collect();
    match operation_choice {
        Ok(choice) => {
            let projects_choice = Select::new("Choose Project", projects).prompt();
            match projects_choice {
                Ok(project) => match choice {
                    "Deploy Application" => {
                        // First Check if the repo exists?
                        let service: Option<&Service> =
                            project_iterator.clone().find(|&s| s.name == project);
                        // dbg!(service.unwrap());
                        let repository_path =
                            config.repository_path.to_owned() + "/" + &service.unwrap().slug;
                        let repo_url = service.unwrap().repository_url.as_str();
                        let repo_exists =
                            check_repository(Path::new(repository_path.clone().as_str()));
                        if !repo_exists {
                            prompt_clone_repository(
                                &git_username,
                                &git_password,
                                &repo_url,
                                &repository_path,
                            )
                        } else {
                        }
                    }
                    "Restart Application" => (),
                    "Stop Application" => (),
                    &_ => {
                        println!("Invalid Flow, please restart the process");
                        exit(1);
                    }
                },
                Err(_) => {
                    println!("There was an error, please try again, choose a project operation")
                }
            };
        }
        Err(_) => println!("There was an error, please try again, choose a valid operation"),
    };
}
