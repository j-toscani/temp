
use std::path::{PathBuf};
use std::error::Error;
mod store;

enum Action {
    CREATE,
    ADD,
    REMOVE
}
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

        let path = match args.len() >= 4 {
            true => PathBuf::from(args[3].as_str()),
            false => PathBuf::from(format!("{}{}", "New", template_key))
        };
        
        Ok(Config {
            action,
            template_key,
            path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.action {
        Action::ADD =>  store::add_to_store(config),
        Action::CREATE => store::create_from_store(config),
        Action::REMOVE => store::remove_from_store(config)
    }
}