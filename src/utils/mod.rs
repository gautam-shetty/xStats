use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::to_writer;
use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::path::Path;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("Failed to read the file {}", file_path))
}

pub fn get_file_name(file_path: &str) -> String {
    Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_file_extension(file_path: &str) -> String {
    if let Some(extension) = Path::new(file_path).extension() {
        format!(".{}", extension.to_str().unwrap())
    } else {
        String::new()
    }
}

pub fn traverse_path(dir_path: &str) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);

    if !path.exists() {
        return Err(format!("Path does not exist: {}", dir_path));
    }

    if path.is_file() {
        files.push(path.to_str().unwrap().to_string());
    } else if path.is_dir() {
        for entry in
            read_dir(path).map_err(|e| format!("Failed to read directory '{}': {}", dir_path, e))?
        {
            let entry =
                entry.map_err(|e| format!("Failed to read entry in '{}': {}", dir_path, e))?;
            let path = entry.path();

            if path.is_dir() {
                files.extend(traverse_path(path.to_str().unwrap())?);
            } else {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    Ok(files)
}

pub fn save_to_csv(file_path: &str, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut writer = Writer::from_path(file_path)?;

    // Write rows to the CSV file
    for row in data {
        writer.write_record(&row)?;
    }

    // Flush to ensure all data is written to the file
    writer.flush()?;
    Ok(())
}

pub fn save_to_json(file_path: &str, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let json_data: Vec<serde_json::Value> = if data.is_empty() {
        Vec::new()
    } else {
        let headers = data[0].clone();
        data[1..]
            .iter()
            .map(|row| {
                let mut map = serde_json::Map::new();
                for (header, value) in headers.iter().zip(row) {
                    map.insert(header.clone(), serde_json::Value::String(value.clone()));
                }
                serde_json::Value::Object(map)
            })
            .collect()
    };

    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(file_path)?;
    to_writer(file, &json_data)?;
    Ok(())
}

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:100} [{pos}/{len}]\n{spinner:.green} {msg}")
            .expect("Failed to set progress bar style"),
    );
    pb
}

pub fn create_multi_commit_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {spinner:.blue} {pos} | {msg}")
            .expect("Failed to set progress bar style"),
    );
    pb
}
