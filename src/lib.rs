use std::error::Error;
use std::path::PathBuf;
mod store;

enum Action {
    CREATE,
    ADD,
    REMOVE,
    LIST,
}
pub struct Config {
    action: Action,
    template_key: String,
    path: PathBuf,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        let action = match args[1].to_lowercase().as_str().trim() {
            "list" => Action::LIST,
            "create" => Action::CREATE,
            "add" => Action::ADD,
            "remove" => Action::REMOVE,
            _ => return Err("This action does not exist"),
        };

        let min_args = match action {
            Action::LIST => 2,
            _ => 3,
        };

        if args.len() < min_args {
            return Err("Please provide an action, a template_key and an outputpath.");
        }

        let template_key = match action {
            Action::LIST => String::from(""),
            _ => String::from(&args[2]),
        };

        let path = match args.len() >= 4 {
            true => PathBuf::from(args[3].as_str()),
            false => PathBuf::from(format!("{}.{}", "New", template_key)),
        };
        Ok(Config {
            action,
            template_key,
            path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut store = store::TemplateStore::new("templates.txt");

    match config.action {
        Action::LIST => Ok(store.list_from_store()),
        Action::ADD => store.add_to_store(config),
        Action::CREATE => store.create_from_store(config),
        Action::REMOVE => store.remove_from_store(config),
    }
}
