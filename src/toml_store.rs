use dirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Template {
    path: PathBuf,
}

impl Template {
    pub fn set(&mut self, path: PathBuf) {
        self.path = path;
    }
}

pub struct TemplateStore {
    _file_path: PathBuf,
    pub store: HashMap<String, Template>,
}

impl TemplateStore {
    pub fn new() -> TemplateStore {
        let _file_path = setup_template_file();
        let content: String = get_store_content(&_file_path);
        let store: HashMap<String, Template> = toml::from_str(&content).unwrap();
        return TemplateStore { _file_path, store };
    }
    pub fn add_template(&mut self, key: String, path: PathBuf) {
        self.store.insert(key, Template { path });
    }
    pub fn remove_template(&mut self, key: String) {
        self.store.remove(&key);
    }
    pub fn update_template(&mut self, key: String, path: PathBuf) {
        if self.store.contains_key(&key) {
            self.store
                .entry(key)
                .and_modify(|template| template.set(path));
        } else {
            println!("Entry with key {} does not exist", key)
        }
    }
    pub fn save(&self) {
        let content: String = toml::to_string(&self.store).unwrap();
        self.write_to_store(content);
    }

    fn write_to_store(&self, content: String) {
        std::fs::write(&self._file_path, content).expect("Unable to write to file.");
    }
}

fn get_store_content(file_path: &PathBuf) -> String {
    read_to_string(&file_path).unwrap()
}

fn setup_template_file() -> PathBuf {
    let mut temp_dir = get_temp_dir();
    temp_dir.push("templates.toml");
    if !temp_dir.exists() {
        write(&temp_dir, "").unwrap();
    }
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
