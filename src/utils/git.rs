use std::{path::Path, process::exit};

use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    AutotagOption, BranchType, Cred, FetchOptions, RemoteCallbacks, Repository,
};
use inquire::Select;

/// Check if the repository exists on the local filesystem
pub fn check_repository(path: &Path) -> bool {
    let repo = Repository::open(path);
    match repo {
        Ok(_repo) => true,
        Err(_) => false,
    }
}
/// handle the user choice and clone the repo as required\
/// wrapper around the git2 library
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
pub fn pull_repository(git_username: &str, git_password: &str, repository_path: &str) -> bool {
    let repo = Repository::open(Path::new(repository_path)).unwrap();
    let mut remote = repo.find_remote("origin").unwrap();
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, _, _| Cred::userpass_plaintext(git_username, git_password));
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);
    fo.download_tags(AutotagOption::All); // Fetch all tags

    // Fetch all branches from the remote
    let fetch_status = remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fo), None);
    if fetch_status.is_err() {
        println!("Failed to fetch from remote: {:?}", fetch_status.err());
        return false;
    }
    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    for branch_result in branches {
        let (branch, branch_type) = branch_result.unwrap();
        if branch_type == BranchType::Local {
            let branch_name = branch.name().unwrap().unwrap_or("<unknown>");

            // Find the corresponding remote-tracking branch
            let upstream_name = format!("refs/remotes/{}", branch_name);
            if let Ok(upstream) = repo.find_reference(&upstream_name) {
                let upstream_commit = upstream.peel_to_commit().unwrap();
                let local_commit = branch.get().peel_to_commit().unwrap();

                // Check if the local branch is behind the remote
                if local_commit.id() != upstream_commit.id()
                    && repo
                        .graph_descendant_of(upstream_commit.id(), local_commit.id())
                        .unwrap()
                {
                    println!("Fast-forwarding branch: {}", branch_name);

                    // Fast-forward the branch
                    let mut branch_ref = branch.into_reference();
                    branch_ref
                        .set_target(upstream_commit.id(), "Fast-forwarding")
                        .unwrap();
                } else {
                    println!(
                        "Branch '{}' is up-to-date or cannot be fast-forwarded.",
                        branch_name
                    );
                }
            } else {
                println!("No upstream branch found for '{}'", branch_name);
            }
        }
    }
    true
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
    branch_list.retain(|branch| branch != "origin/HEAD");
    let filtered_branch_list: Vec<&str> =
        branch_list.iter().map(|branch| branch.as_str()).collect();
    // prompt the user to select the branch
    let branch_selection =
        Select::new("choose the branch to be deployed", filtered_branch_list).prompt();

    match branch_selection {
        Ok(branch) => Some(branch.to_owned()),
        Err(_) => {
            println!("There was an error, please try again, choose a valid branch");
            None
        }
    }
}

pub fn branch_checkout(repository_path: &str, branch_selection: String) {
    let repo = Repository::open(Path::new(repository_path)).unwrap();
    let remote_branch_ref = repo
        .find_branch(branch_selection.as_str(), BranchType::Remote)
        .unwrap();
    let remote_branch_commit = repo
        .reference_to_annotated_commit(remote_branch_ref.get())
        .unwrap();

    // Get the actual commit from the annotated commit
    let commit = repo.find_commit(remote_branch_commit.id()).unwrap();

    if let Ok(local_branch) = repo.find_branch(branch_selection.as_str(), BranchType::Local) {
        // Checkout the existing local branch
        repo.set_head(local_branch.get().name().unwrap()).unwrap();
    } else {
        // Create a new local branch that tracks the remote branch
        let local_branch = repo
            .branch(branch_selection.as_str(), &commit, false)
            .unwrap();
        repo.set_head(local_branch.get().name().unwrap()).unwrap();
    }

    // Checkout the new local branch
    let (object, reference) = repo.revparse_ext(branch_selection.as_str()).unwrap();
    repo.checkout_tree(&object, Some(&mut CheckoutBuilder::new().force()))
        .unwrap();
    repo.set_head(reference.unwrap().name().unwrap()).unwrap();
}
