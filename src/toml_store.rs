use dirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use toml;

#[derive(Deserialize)]
pub struct Template {
    path: PathBuf,
}

pub struct TemplateStore {
    store: HashMap<String, Template>,
}

impl TemplateStore {
    pub fn new() -> TemplateStore {
        let mut dir: PathBuf = get_temp_dir();
        let file_path = get_file_path(&mut dir);

        println!("{:?}", file_path);

        if !file_path.exists() {
            write(&file_path, "").unwrap();
        }

        let content = read_to_string(file_path).unwrap();
        let store: HashMap<String, Template> = toml::from_str(&content).unwrap();

        return TemplateStore { store };
    }
}

fn get_file_path(temp_dir: &mut PathBuf) -> &mut PathBuf {
    temp_dir.push("templates.toml");
    temp_dir
}

fn get_temp_dir() -> PathBuf {
    let mut home = dirs::home_dir().unwrap();
    home.push("temp/");
    println!("{:?}", home);

    match create_dir_all(&home) {
        Ok(_) => home,
        Err(_) => home,
    }
}
