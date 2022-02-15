use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::Config;
use std::env::current_exe;
use std::error::Error;
use std::fs::{copy, create_dir_all, remove_file, write, File, OpenOptions};
use std::io::{BufRead, BufReader, Read};

struct TemplateStore {
    file: File,
    path: PathBuf,
    values: HashMap<String, String>,
}

impl TemplateStore {
    fn new(filename: &str) -> TemplateStore {
        let (file, path) = get_template_file(filename).unwrap();

        let reader = BufReader::new(&file);
        let mut values: HashMap<String, String> = HashMap::new();
        let valid_lines = validate_lines(reader);
        lines_to_hashmap(valid_lines, &mut values);

        TemplateStore { file, path, values }
    }
}

pub fn add_to_store(config: Config) -> Result<(), Box<dyn Error>> {
    let mut store = TemplateStore::new("templates.txt");
    if store.values.contains_key(&config.template_key) {
        panic!("Template already existing.");
    };
    let template = get_template_to_add(&config.path)?;
    writeln!(store.file, "{} {}", config.template_key, template)?;
    Ok(())
}

pub fn remove_from_store(config: Config) -> Result<(), Box<dyn Error>> {
    let mut store = TemplateStore::new("templates.txt");
    let mut tmp_store = TemplateStore::new("tmp_templates.txt");

    match store.values.remove_entry(&config.template_key) {
        Some(entry) => println!("Removed '{}' from store.", entry.0),
        None => println!(
            "Template with key '{}' does not exist.",
            &config.template_key
        ),
    };
    for key in store.values.keys() {
        writeln!(tmp_store.file, "{} {}", key, store.values.get(key).unwrap())?;
    }

    copy(&tmp_store.path, store.path)?;
    remove_file(tmp_store.path)?;

    Ok(())
}

pub fn list_from_store() {
    let store = TemplateStore::new("templates.txt");
    println!("The following keys are registered: ");
    for key in store.values.keys() {
        println!("{}", key)
    }
}

pub fn create_from_store(config: Config) -> Result<(), Box<dyn Error>> {
    let store = TemplateStore::new("templates.txt");
    let template = store
        .values
        .get(&config.template_key)
        .expect("Template not found");
    let parent = config.path.parent().unwrap_or(Path::new("/"));

    if !parent.exists() {
        create_dir_all(parent)?;
    }

    let saveable_template = template.replace("<?>", "\n");

    write(config.path, saveable_template)?;
    Ok(())
}

fn get_template_to_add(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut template = File::open(path)?;
    let mut template_string = String::new();
    template.read_to_string(&mut template_string)?;
    let saveable_template = template_string.replace("\n", "<?>");
    Ok(saveable_template)
}

fn get_template_file(filename: &str) -> Result<(File, PathBuf), Box<dyn Error>> {
    let mut template_file_path: PathBuf = current_exe()?;
    template_file_path.set_file_name(filename);
    let file = get_writable_file(&template_file_path)?;
    Ok((file, template_file_path))
}

fn get_writable_file(filepath: &PathBuf) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(filepath)
}

fn validate_lines(reader: BufReader<&File>) -> Vec<String> {
    let valid_lines: Vec<String> = reader
        .lines()
        .map(|line| line.ok())
        .map(|line| line.unwrap())
        .collect();
    valid_lines
}

fn lines_to_hashmap(valid_lines: Vec<String>, store: &mut HashMap<String, String>) {
    for line in valid_lines {
        let split_line = line.split_once(" ");
        if split_line.is_some() {
            let (key, value) = split_line.unwrap();
            store.insert(String::from(key), String::from(value));
        }
    }
}
