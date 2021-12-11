use std::path::{PathBuf, Path};
use std::error::Error;
use std::fs::{create_dir_all, write};

#[derive(Debug)]
enum Action {
    CREATE,
    ADD,
}

#[derive(Debug)]
pub struct Config {
    action: Action,
    // template_key: String,
    outpath: PathBuf,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("Please provide an action, a template_key and an outputpath.");
        }

        let action = match args[1].as_str().trim() {
            "create" => Action::CREATE,
            "add" => Action::ADD,
            _ => return Err("This this action does not exist"),
        };

        // let template_key = String::from(args[2]);

        let mut outpath = PathBuf::from(args[3].as_str());

        let filename = match args.len() >= 5 {
            true => args[4].as_str(),
            false => "New.txt",
        };

        outpath.push(filename);

        Ok(Config {
            action,
            // template_key,
            outpath,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let parent = match config.outpath.parent() {
        Some(path) => path,
        None => Path::new("/")
    };

    if !parent.exists() {
        create_dir_all(parent)?;
    }

    match config.action {
        Action::ADD => println!("Using Create..."),
        Action::CREATE => write(config.outpath, "Hello!")?
    }

    Ok(())
}