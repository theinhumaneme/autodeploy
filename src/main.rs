use inquire::InquireError;
use inquire::Select;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::process::exit;
use text_to_ascii_art::to_art;
use toml;

// USER FLOW
// prompt for operation, deploy restart or stop
// prompt for the appropriate project
// prompt for branch in the project if operation is deployment
// else use the docker bindings or the os to operate using the files generated
//

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfiguration {
    print_banner: bool,
    client: Option<String>,
    organization: Option<String>,
    configuration_file: Option<String>,
}
fn init() {
    let global_configuration_file = "global.toml";
    let conifg = match fs::read_to_string(global_configuration_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", global_configuration_file);
            exit(1);
        }
    };
    let config: GlobalConfiguration = match toml::from_str(&conifg) {
        Ok(d) => d,
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
}
fn main() {
    init();
    let operations: Vec<&str> = vec![
        "Deploy Application",
        "Restart Application",
        "Stop Application",
    ];
    let projects: Vec<&str> = vec![];
    let operation_choice: Result<&str, InquireError> =
        Select::new("What would you like to do?", operations).prompt();
    match operation_choice {
        Ok(choice) => {
            let projects_choice = Select::new("Choose Project", projects).prompt();
            match projects_choice {
                Ok(project) => match choice {
                    "Deploy Application" => (),
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
