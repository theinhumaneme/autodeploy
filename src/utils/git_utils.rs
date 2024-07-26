use git2::{Branches, Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

pub fn clone_repo(git_username: &str, git_password: &str) {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext(git_username, git_password)
    });
    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    builder.clone("", Path::new(""));
}
