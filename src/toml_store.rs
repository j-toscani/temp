use dirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Debug)]
pub struct Template {
    path: PathBuf,
}

impl Template {
    pub fn set(&mut self, path: PathBuf) {
        self.path = path;
    }
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
    pub fn add(&mut self, key: String, path: PathBuf) {
        self.store.insert(key, Template { path });
    }
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
    pub fn update(&mut self, key: String, path: PathBuf) {
        if self.store.contains_key(&key) {
            self.store
                .entry(key)
                .and_modify(|template| template.set(path));
        } else {
            println!("Entry with key {} does not exist", key)
        }
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
