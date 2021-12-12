use std::path::{PathBuf, Path};
use std::error::Error;
use std::env::current_exe;
use std::io::{BufRead, BufReader};
use std::fs::{create_dir_all, write, File, remove_file};

#[derive(Debug)]
enum Action {
    CREATE,
    // ADD,
}

#[derive(Debug)]
pub struct Config {
    action: Action,
    template_key: String,
    outpath: PathBuf,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("Please provide an action, a template_key and an outputpath.");
        }

        let action = match args[1].as_str().trim() {
            "create" => Action::CREATE,
            // "add" => Action::ADD,
            _ => return Err("This this action does not exist"),
        };

        let template_key = String::from(&args[2]);

        let mut outpath = PathBuf::from(args[3].as_str());

        let filename = match args.len() >= 5 {
            true => args[4].as_str(),
            false => "New.txt",
        };

        outpath.push(filename);

        Ok(Config {
            action,
            template_key,
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
        // Action::ADD => ,
        Action::CREATE => create_file_from_template(config)
    }
}

fn create_file_from_template(config : Config) -> Result<(), Box<dyn Error>>{
    let template_file_name = "templates.txt";
    let file = get_template_file(template_file_name)?;

    let template = match get_template_from_file(file, &config.template_key) {
        Some(line) => line,
        None => {
            println!("Requested template does not exist.");
            return Ok(())
        }
    };


    write(config.outpath, template)?;
    Ok(())
}

fn get_template_file(filename: &str) -> Result<File, std::io::Error>{
    let mut template_file_path: PathBuf = current_exe()?; 
    template_file_path.set_file_name(filename);

    if template_file_path.exists() {
        File::open(&template_file_path)
    } else {
        File::create(&template_file_path)?;
        File::open(template_file_path)
    }
}

fn get_template_from_file(file: File, template_key: &String) -> Option<String> {
    let reader = BufReader::new(file);
    let mut result : Option<String> = None;

    for (_index, line) in reader.lines().enumerate() {
        let line: String = line.unwrap();
        let line_hase_template = line.starts_with(template_key);

        if line_hase_template {
            let template_start = match line.find(" ") {
                Some(pos) => pos + " ".len(),
                None => 0
            };
        
            result = match line.get(template_start..) {
                Some(string) => Some(String::from(string)),
                None => None
            };

            break;
        }
    }

    result
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
            remove_file(&template_file_path).unwrap();
        }
    }

    #[test]
    fn open_file() {
        let filename = "template-test.txt";
        create_test_file(&filename);
        let file = get_template_file(&filename);

        assert_eq!(file.is_ok(), true);
    }
    #[test]
    fn create_if_nonexistent_and_open() {
        let filename = "template-test.txt";
        remove_test_file(&filename);

        let file = get_template_file(&filename);

        assert_eq!(file.is_ok(), true)
    }
    #[test]
    fn key_does_not_exist() {
        let filename = "template-test.txt";
        let test_file = create_test_file(&filename);

        let template_key = String::from("vue");

        let template = get_template_from_file(test_file, &template_key);
        assert_eq!(None, template);
    }
    #[test]
    fn key_does_exist() {
        let filename = "template-test.txt";
        let test_file = create_test_file(&filename);

        let template_key = String::from("txt");

        let template = get_template_from_file(test_file, &template_key);
        assert_eq!(Some(String::from("Hello World!")), template);
    }
}