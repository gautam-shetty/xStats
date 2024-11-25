use std::fs;
use std::fs::read_dir;
use std::path::Path;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("Failed to read the file {}", file_path))
}

pub fn traverse_dir(dir_path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);

    if path.is_dir() {
        for entry in read_dir(path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.is_dir() {
                files.extend(traverse_dir(path.to_str().unwrap()));
            } else {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    files
}
