pub fn to_absolute_path(path: &str) -> std::path::PathBuf {
    // Fix needed to run AppImage on Linux
    let original_dir = std::env::var("OWD");

    let current_dir = match original_dir {
        Ok(dir) => std::path::PathBuf::from(dir),
        Err(_) => std::env::current_dir().expect("Could not get current directory"),
    };

    return current_dir.join(path);
}
