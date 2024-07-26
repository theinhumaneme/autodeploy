use git2::{Branches, Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

pub fn check_repo(path: &Path) -> bool {
    let repo = Repository::open(path);
    match repo {
        Ok(repo) => return true,
        Err(_) => return false,
    }
}

pub fn clone_repo(git_username: &str, git_password: &str, repo_url: &str, path: &str) {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, _, _| Cred::userpass_plaintext(git_username, git_password));
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    builder.clone(repo_url, Path::new(path));
}
