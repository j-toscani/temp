use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::env::current_exe;


pub fn get_store_file(template_file_path: PathBuf) -> Result<File, std::io::Error> {
    std::fs::OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(template_file_path)
}

pub fn get_path_to_store(filename: &str) -> Result<PathBuf, Box<dyn Error>>{
    let mut template_file_path: PathBuf = current_exe()?;
    template_file_path.set_file_name(filename);
    return Ok(template_file_path)
}

pub fn get_store_entry(file: &File, template_key: &String) -> Option<String> {
    let reader = BufReader::new(file);
    let mut result: Option<String> = None;

    for (_index, line) in reader.lines().enumerate() {
        let line: String = match line {
            Ok(line) => line,
            Err(_) => return None,
        };

        if line.starts_with(template_key) {
            result = Some(line);
        }
    }

    result
}

pub fn get_store_entry_value(line: &String) -> String {
    let template_start = line.find(" ").expect("Template not saved correctly.");
    let template = line.get(template_start..).unwrap_or("").trim();
    String::from(template)
}
