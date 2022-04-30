use dirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, write, File, OpenOptions};
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
    _file_path: PathBuf,
    store: HashMap<String, Template>,
}

impl TemplateStore {
    pub fn new(&self) -> TemplateStore {
        let temp_dir = get_temp_dir();
        setup_template_file(&mut temp_dir);
        let content: String = self.get_store_content();
        let store: HashMap<String, Template> = toml::from_str(&content).unwrap();

        return TemplateStore {
            _file_path: temp_dir,
            store,
        };
    }
    fn get_store_content(&self) -> String {
        read_to_string(self._file_path).unwrap()
    }
    fn write_to_store(&self) -> File {
        OpenOptions::new()
            .write(true)
            .open(self._file_path)
            .unwrap()
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

fn setup_template_file(temp_dir: &mut PathBuf) -> &PathBuf {
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
