
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::io::Write;

use std::env::current_exe;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::fs::{File, OpenOptions, create_dir_all, write, copy, remove_file};
use crate::Config;

struct TemplateStore {
    file: File,
    path: PathBuf,
    values: HashMap<String, String>
}

impl TemplateStore {
    fn new(filename: &str) -> TemplateStore {
        let file_and_path = get_template_file(filename).unwrap();
        let template_store = get_template_store(&file_and_path.0).unwrap();

        TemplateStore {
            file: file_and_path.0,
            path: file_and_path.1,
            values: template_store
        }
    }
}
pub struct TemplateLine {
    exists: bool,
    key: String,
    template: String
}

impl TemplateLine {
    fn new(line: String) -> TemplateLine{
        let (key, template, exists) = match line.split_once(" ") {
            Some((key, template)) => (key, template, true),
            None => ("", "", false)
        };

        TemplateLine {
            key: String::from(key), 
            template: String::from(template),
            exists
        }
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
    let store = TemplateStore::new("templates.txt");
    let mut tmp_store = TemplateStore::new("tmp_templates.txt");

    if !store.values.contains_key(&config.template_key) {
        println!("Key '{}' does not exist.", &config.template_key);
        return Ok(())
    }
    
    for key in store.values.keys() {
        writeln!(tmp_store.file, "{} {}", key, store.values.get(key).unwrap())?;
    }

    copy(&tmp_store.path, store.path)?;
    remove_file(tmp_store.path)?;

    Ok(())
}

pub fn create_from_store(config: Config) -> Result<(), Box<dyn Error>> {
    let store = TemplateStore::new("templates.txt");
    let template = store.values.get(&config.template_key).expect("Template not found");
    let parent = config.path.parent().unwrap_or(Path::new("/"));

    if !parent.exists() {
        create_dir_all(parent)?;
    }

    write(config.path, template)?;
    Ok(())
}

fn get_template_to_add(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut template = File::open(path)?;
    let mut template_string = String::new();
    template.read_to_string(&mut template_string)?;

    Ok(template_string)
}

fn get_template_file(filename: &str) -> Result<(File, PathBuf), Box<dyn Error>>{
    let mut template_file_path: PathBuf = current_exe()?; 
    template_file_path.set_file_name(filename);
    let file = get_writable_file(&template_file_path)?;
    Ok((file, template_file_path))
}

fn get_template_store(file: &File) -> Result<HashMap<String, String>, Box<dyn Error>>{
    let template_lines = collect_template_lines(file);
    Ok(create_template_store(template_lines))
}

fn get_writable_file(filepath: &PathBuf) -> Result<File, std::io::Error>{
    OpenOptions::new().append(true).create(true).read(true).open(filepath)
}

fn create_template_store(lines: Vec<TemplateLine>) -> HashMap<String, String> {
    let mut store: HashMap<String, String> = HashMap::new();
    let filtered_templates = lines.iter().filter(|template| template.exists);

    for template in filtered_templates {
        store.insert(template.key.clone(), template.template.clone());
    }
    store
}

fn collect_template_lines(file: &File) -> Vec<TemplateLine> {
    let reader = BufReader::new(file);

    let valid_lines: Vec<TemplateLine> = reader.lines()
    .map(|line|line.ok())
    .map(|line| TemplateLine::new(line.unwrap_or(String::from("--"))))
    .collect();
    valid_lines
}
