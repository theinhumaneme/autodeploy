use git2::build::{self, RepoBuilder};
use git2::{AutotagOption, BranchType, Cred, FetchOptions, RemoteCallbacks, Repository};
use inquire::Select;
use std::path::Path;
use std::process::exit;

/// Check if the repository exists on the local filesystem
pub fn check_repository(path: &Path) -> bool {
    let repo = Repository::open(path);
    match repo {
        Ok(_repo) => return true,
        Err(_) => return false,
    }
}
///
/// handle the user choice and clone the repo as required\
/// wrappper around the git2 library
pub fn prompt_clone_repository(
    git_username: &str,
    git_password: &str,
    repo_url: &str,
    repository_path: &str,
) {
    let clone_allow = Select::new(
        "Repository does not seem to exist.\nWould you like to clone it",
        vec!["Yes", "No"],
    )
    .prompt();
    match clone_allow {
        Ok(clone_allow_option) => {
            if clone_allow_option == "Yes" {
                println!("Cloning in progress");
                let mut callbacks = RemoteCallbacks::new();
                callbacks
                    .credentials(|_, _, _| Cred::userpass_plaintext(git_username, git_password));
                let mut fo = FetchOptions::new();
                fo.remote_callbacks(callbacks);
                let mut builder = RepoBuilder::new();
                builder.fetch_options(fo);
                let clone_status = builder.clone(repo_url, Path::new(repository_path));
                match clone_status {
                    Ok(_repo) => {
                        println!("Cloning repository is complete");
                    }
                    Err(_) => {
                        eprintln!("FATAL Cloning the repostiory failed");
                    }
                }
            } else if clone_allow_option == "No" {
                eprintln!("Please clone the repo manually at {repository_path} to proceed with deployment");
                exit(0);
            }
        }
        Err(_) => println!("There was an error, please try again, choose a valid option"),
    }
}

/// check if the reposity can be pulled from the remote.
pub fn pull_repository(
    git_username: &str,
    git_password: &str,
    repo_url: &str,
    repository_path: &str,
) -> bool {
    let repo = Repository::open(Path::new(repository_path)).unwrap();
    let mut remote = repo.find_remote("origin").unwrap();
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, _, _| Cred::userpass_plaintext(git_username, git_password));
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);
    fo.download_tags(AutotagOption::All); // Fetch all tags

    // Fetch all branches from the remote
    let fetch_status = remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fo), None);
    let pull_status = match fetch_status {
        Ok(_) => true,
        Err(_) => false,
    };
    println!("All branches have been fetched and updated successfully.");
    pull_status
}

/// prompt the user to select a branch.
pub fn prompt_branch_selection(repository_path: &str) -> Option<String> {
    // first fetch the repo and then prompt the branches for selection
    // repo is assumed to exist on the filesystem
    let repo = Repository::open(repository_path).unwrap();
    let branches = repo.branches(Some(BranchType::Remote));
    let mut branch_list: Vec<String> = vec![];
    match branches {
        Ok(branches) => {
            for branch_result in branches {
                match branch_result {
                    Ok((branch, _)) => {
                        match branch.name() {
                            Ok(Some(name)) => branch_list.push(name.to_string()),
                            Ok(None) => {
                                println!("cannot parse/fetch the branch try again")
                            }
                            Err(e) => {
                                // Handle error from branch.name()
                                eprintln!("Error getting branch name: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        // Handle error from branch_result
                        eprintln!("Error processing branch result: {:?}", e);
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Cannot fetch branches");
        }
    }
    // remove origin/HEAD from the listed options
    branch_list = branch_list
        .into_iter()
        .filter(|branch| branch != "origin/HEAD")
        .collect();
    let filtered_branch_list: Vec<&str> =
        branch_list.iter().map(|branch| branch.as_str()).collect();
    // prompt the user to select the branch
    let branch_selection =
        Select::new("choose the branch to be deployed", filtered_branch_list).prompt();
    let selected_branch = match branch_selection {
        Ok(branch) => Some(branch.to_owned()),
        Err(_) => {
            println!("There was an error, please try again, choose a valid branch");
            None
        }
    };
    selected_branch
}
