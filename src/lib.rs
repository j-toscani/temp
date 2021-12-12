use std::path::{PathBuf, Path};
use std::error::Error;
use std::env::current_exe;
use std::io::{BufRead, BufReader};
use std::fs::{create_dir_all, write, File, OpenOptions};

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

        let filename = match args.len() >= 5 {
            true => args[4].as_str(),
            false => "New.txt",
        };

        path.push(filename);

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
        Action::CREATE => create_file_from_template(config)
    }
}

fn add_template(config: Config) -> Result<(), Box<dyn Error>> {
    let template_file = get_template_file("templates.txt",true)?;

    let template_exists = match find_template_entry(template_file, &config.template_key) {
        Some(line) => true,
        None => false
    };

    if template_exists {
        println!("Template already created. To replace it, you have to remove it first.");
        return Ok(())
    };

    if !config.path.is_file() {
        println!("The provided path is not pointing to a file.");
        return Ok(())
    }

    let content = std::fs::read_to_string(config.path)?;
    write!(template_file, "{} {}", config.template_key, content)
}

fn create_file_from_template(config : Config) -> Result<(), Box<dyn Error>>{
    let file = get_template_file("templates.txt", false)?;

    let template = match find_template_entry(file, &config.template_key) {
        Some(line) => get_template_from_line(&line),
        None => {
            println!("Requested template does not exist.");
            return Ok(())
        }
    };

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

fn get_template_file(filename: &str, append: bool) -> Result<File, std::io::Error>{
    let mut template_file_path: PathBuf = current_exe()?; 
    template_file_path.set_file_name(filename);

    OpenOptions::new().append(append).create(true).write(true).open(template_file_path)
}

fn find_template_entry(file: File, template_key: &String) -> Option<String> {
    let reader = BufReader::new(file);
    let mut result : Option<String> = None;

    for (_index, line) in reader.lines().enumerate() {
        let line: String = line.unwrap();

        if line.starts_with(template_key) {
            result = Some(line);
            break;
        }
    }

    result
}

fn get_template_from_line(line: &String) -> String {
    let template_start = line.find(" ").expect("Template not saved correctly.");
    let template = line.get(template_start..).unwrap().trim();
    String::from(template)
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn create_test_file(filename: &str) -> File {
        let content = "txt Hello World!\n";
        let mut template_file_path: PathBuf = current_exe().unwrap();
        template_file_path.set_file_name(&filename);

        write(&template_file_path, content).unwrap();
        File::open(template_file_path).unwrap()
    }

    fn remove_test_file(filename: &str) {
        let mut template_file_path: PathBuf = current_exe().unwrap();
        template_file_path.set_file_name(filename);

        if template_file_path.exists() {
            std::fs::remove_file(&template_file_path).unwrap();
        }
    }
}