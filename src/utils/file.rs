use std::path::Path;
pub fn check_file(path: &str) -> bool {
    if Path::exists(Path::new(path)) {
        true
    } else {
        println!("specified file does not exist at given path {}", path);
        false
    }
}
