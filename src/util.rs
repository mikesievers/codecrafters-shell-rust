use is_executable::IsExecutable;
use std::env;
use std::fs;

pub fn find_executable_in_path(cmd_name: &String) -> Option<String> {
    // Iterate over all paths in os env variable PATH
    // and check whether an executable of that name is in the path
    // Return the full path if found, None if not

    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let full_path = path.join(cmd_name);
                if let Ok(file_exists) = fs::exists(&full_path) {
                    if file_exists && full_path.is_executable() {
                        return Some(full_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
        None => {
            println!("OS environment variable PATH is not set.");
        }
    }

    // No matching path has been found
    None
}
