pub fn to_absolute_path(path: &str) -> std::path::PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    return current_dir.join(path);
}
