use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::error::Error;
use std::env::current_exe;
use std::io::{BufRead, BufReader};
// use std::io::Write;
use std::fs::{create_dir_all, File, write};

#[derive(Debug)]
enum Action {
    CREATE,
    // ADD,
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
        if args.len() < 4 {
            return Err("Please provide an action, a template_key and an outputpath.");
        }

        let action = match args[1].as_str().trim() {
            "create" => Action::CREATE,
            // "add" => Action::ADD,
            _ => return Err("This action does not exist"),
        };

        let template_key = String::from(&args[2]);

        let mut path = PathBuf::from(args[3].as_str());

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
    match config.action {
        // Action::ADD => add_template(config),
        Action::CREATE => create_file_from_template(config)
    }
}

fn create_file_from_template(config : Config) -> Result<(), Box<dyn Error>>{
    let template_store = get_template_store()?;
    let template = template_store.get(&config.template_key).unwrap();

    let parent = match config.path.parent() {
        Some(path) => path,
        None => Path::new("/")
    };

    if !parent.exists() {
        create_dir_all(parent)?;
    }

    write(config.path, template)?;
    Ok(())
}

fn get_template_store() -> Result<HashMap<String, String>, Box<dyn Error>>{
    let file = get_template_file()?;
    let template_lines = collect_template_lines(&file);
    Ok(create_template_store(template_lines))
}

fn get_template_file() -> Result<File, std::io::Error>{
    let mut template_file_path: PathBuf = current_exe()?; 
    template_file_path.set_file_name("templates.txt");

    if template_file_path.exists() {
        File::open(&template_file_path)
    } else {
        File::create(&template_file_path)?;
        File::open(&template_file_path)
    }
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