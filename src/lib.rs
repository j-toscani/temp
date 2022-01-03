
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::error::Error;
use std::env::current_exe;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::{create_dir_all, File, write, copy, remove_file, OpenOptions};

#[derive(Debug)]
enum Action {
    CREATE,
    ADD,
    REMOVE
}

struct TemplateLine {
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

#[derive(Debug)]
pub struct Config {
    action: Action,
    template_key: String,
    path: PathBuf,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Please provide an action, a template_key and an outputpath.");
        }

        let action = match args[1].to_lowercase().as_str().trim() {
            "create" => Action::CREATE,
            "add" => Action::ADD,
            "remove" => Action::REMOVE,
            _ => return Err("This action does not exist"),
        };

        let template_key = String::from(&args[2]);

        let mut path = match args.len() >= 4 {
            true => PathBuf::from(args[3].as_str()),
            false => PathBuf::from("")
        };

        if !path.is_file() {
            let filename = match args.len() >= 5 {
                true => args[4].as_str(),
                false => "New",
            };

            path.set_file_name(&filename);
            path.set_extension(&template_key);
        }

        Ok(Config {
            action,
            template_key,
            path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut template_file_path: PathBuf = current_exe()?; 
    template_file_path.set_file_name("templates.txt");
    let mut file = get_writable_file(&template_file_path)?;

    let template_store = get_template_store(&file)?;

    match config.action {
        Action::ADD =>  {
            if template_store.contains_key(&config.template_key) {
                panic!("Template already existing.");
            }
            
            let template = get_template_to_add(&config.path)?;
            writeln!(file, "{} {}", config.template_key, template)?;
            
        },
        Action::CREATE => {
            let template = template_store.get(&config.template_key).expect("Template not found");
            let parent = config.path.parent().unwrap_or(Path::new("/"));
        
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        
            write(config.path, template)?;
        }
        Action::REMOVE => {
            if !template_store.contains_key(&config.template_key) {
                println!("Key '{}' does not exist.", &config.template_key);
                return Ok(())
            }
            let mut tmp_template_file_path = template_file_path.clone();
            tmp_template_file_path.set_file_name("tmptemplate.txt");

            let mut tmp_file = get_writable_file(&tmp_template_file_path)?;
        
            for key in template_store.keys() {
                if key != &config.template_key {
                    writeln!(tmp_file, "{} {}", key, template_store.get(key).unwrap())?;
                }
            }
        
            copy(&tmp_template_file_path, template_file_path)?;
            remove_file(tmp_template_file_path)?;
        }
    }
    Ok(())
}


fn get_template_to_add(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut template = File::open(path)?;
    let mut template_string = String::new();
    template.read_to_string(&mut template_string)?;

    Ok(template_string)
}

fn get_template_store(file: &File) -> Result<HashMap<String, String>, Box<dyn Error>>{
    let template_lines = collect_template_lines(file);
    Ok(create_template_store(template_lines))
}

fn get_writable_file(filepath: &PathBuf) -> Result<File, std::io::Error>{
    OpenOptions::new().append(true).create(true).read(true).open(filepath)
}

fn collect_template_lines(file: &File) -> Vec<TemplateLine> {
    let reader = BufReader::new(file);

    let valid_lines: Vec<TemplateLine> = reader.lines()
    .map(|line|line.ok())
    .map(|line| TemplateLine::new(line.unwrap_or(String::from("--"))))
    .collect();
    valid_lines
}

fn create_template_store(lines: Vec<TemplateLine>) -> HashMap<String, String> {
    let mut store: HashMap<String, String> = HashMap::new();
    let filtered_templates = lines.iter().filter(|template| template.exists);

    for template in filtered_templates {
        store.insert(template.key.clone(), template.template.clone());
    }
    store
}