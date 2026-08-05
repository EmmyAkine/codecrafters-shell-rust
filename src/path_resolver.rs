
use std::env;
use std::path::{Path};
#[allow(unused_imports)]
use std::fs;

#[derive(Clone)]
pub struct PathResolver{
    directories: Vec<String>,
}

impl PathResolver{
    fn new(directories: Vec<String>) -> Self {
        PathResolver {directories}
    }
    pub fn new_from_environment() -> Self {
        let path_var:Vec<String> = env::var_os("PATH").map(|os_path|{
            env::split_paths(&os_path).filter_map(|path| path.to_str().map(String::from)).collect()
        }).unwrap_or_default();
        if path_var.is_empty() {
            panic!("PATH environment variable is empty or not set.")
        }

        PathResolver::new(path_var)
    }

    pub fn resolve(&self, name: &String) -> Option<String> {
        for dir in  self.directories.iter(){
            let full_path = Path::new(dir).join(name);
            if let Some(resolved) = Self::is_executable(&full_path) {
                return Some(resolved);
            }
        }
        None
    }
    #[cfg(not(unix))]
    fn is_executable<P: AsRef<Path>>(path: P) -> Option<String> {
        let path = path.as_ref();

        // 1. Get PATHEXT list
        let pathext = env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
        let extensions: Vec<&str> = pathext.split(';').map(|s| s.trim()).collect();

        // CASE A: The path ALREADY has an extension (e.g., "cmd.exe")
        if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
            if path.is_file() {
                // Ensure its extension is actually in PATHEXT
                let matches_pathext = extensions.iter().any(|ext| {
                    ext.trim_start_matches('.').eq_ignore_ascii_case(file_ext)
                });
                if matches_pathext {
                    return Some(path.to_str().unwrap().to_string());
                }
            }
            return None;
        }
        // CASE B: The path HAS NO extension (e.g., "cmd")
        // Try appending each extension from PATHEXT to see if a valid file exists!
        for ext in extensions {
            let clean_ext = ext.trim_start_matches('.').to_lowercase();
            let path_with_ext = path.with_extension(clean_ext);

            if path_with_ext.is_file() {
                return Some(path_with_ext.to_str().unwrap().to_string()); // Found a matching executable
            }
        }
        None
    }

    #[cfg(unix)]
    fn is_executable<P: AsRef<Path>>(path: P) -> Option<String> {
        use std::os::unix::fs::PermissionsExt;

        let path = path.as_ref();

        if !path.is_file() {
            return None;
        }

        // Check if any execute bit (User, Group, Other) is set on `path`
        if fs::metadata(path).map(|meta| (meta.permissions().mode() & 0o111) != 0)
            .unwrap_or(false){
            return  Some(path.to_str().unwrap().to_string());
        }
        None
    }


}