use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};
use std::io::Write;
use std::error::Error;

mod store;

#[derive(Debug)]
enum Action {
    CREATE,
    ADD,
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
            "add" => Action::ADD,
            _ => return Err("This action does not exist"),
        };

        let template_key = String::from(&args[2]);

        let mut path = PathBuf::from(args[3].as_str());

        if !path.is_file() {
            let filename = match args.len() >= 5 {
                true => args[4].as_str(),
                false => "New.txt",
            };

            path.push(filename);
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
        Action::ADD => add_template(config),
        Action::CREATE => create_file_from_template(config),
    }
}

fn create_file_from_template(config: Config) -> Result<(), Box<dyn Error>> {
    let path = store::get_path_to_store("templates.txt")?;
    let file = store::get_store_file(path)?;

    let template = match store::get_store_entry(&file, &config.template_key) {
        Some(line) => store::get_store_entry_value(&line),
        None => {
            println!("Requested template does not exist.");
            return Ok(());
        }
    };

    let parent = config.path.parent().unwrap_or(Path::new("/"));

    if !parent.exists() {
        create_dir_all(parent)?;
    }

    write(config.path, template)?;
    Ok(())
}

fn add_template(config: Config) -> Result<(), Box<dyn Error>> {
    let path = store::get_path_to_store("templates.txt")?;
    let mut file = store::get_store_file(path)?;

    let has_entry = store::get_store_entry(&file, &config.template_key);

    if has_entry.is_some() {
        println!("Entry '{}' exists already.", &config.template_key);
        return Ok(());
    }

    let template = std::fs::read_to_string(&config.path)?;
    writeln!(file, "{} {}", &config.template_key, template)?;
    Ok(())
}

