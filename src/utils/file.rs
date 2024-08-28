use std::path::Path;
pub fn check_file(path: String) -> bool {
    if Path::exists(Path::new(path.as_str())) {
        return true;
    } else {
        println!("specified file does not exist at given path {}", path);
        return false;
    }
}
