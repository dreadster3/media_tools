use std::fs;

use log::debug;

pub fn to_absolute_path(path: &str) -> std::path::PathBuf {
    // Fix needed to run AppImage on Linux
    let original_dir = std::env::var("OWD");

    debug!("Original working directory: {:?}", original_dir);

    let current_dir = match original_dir {
        Ok(dir) => std::path::PathBuf::from(dir),
        Err(_) => std::env::current_dir().expect("Could not get current directory"),
    };

    let full_path = current_dir.join(path);

    let absolute_path = match fs::canonicalize(&full_path) {
        Ok(path) => path,
        Err(_) => full_path,
    };

    return absolute_path;
}

pub fn normalize_command(command: &str) -> String {
    let app_dir = std::env::var("APPDIR");

    debug!("App dir: {:?}", app_dir);

    return match app_dir {
        Ok(dir) => format!("{}/usr/bin/{}", dir, command),
        Err(_) => command.to_string(),
    };
}
