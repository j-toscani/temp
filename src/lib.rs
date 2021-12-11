use std::path::PathBuf;

enum Action {
    CREATE,
    ADD,
}

pub struct Config {
    action: Action,
    template_key: String,
    outpath: PathBuf,
    filename: String,
}

impl Config {
    pub fn new(args: Vec<&str>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Please provide an action and a template_key.");
        }

        let action = match args[0] {
            "create" => Action::CREATE,
            "add" => Action::ADD,
            _ => return Err("This this action does not exist"),
        };

        let template_key = String::from(args[1]);

        let outpath = match args.len() >= 3 {
            true => PathBuf::from(args[2]),
            false => return Err("No Output directory specified."),
        };

        let filename = match args.len() >= 4 {
            true => args[3],
            false => "New",
        };

        Ok(Config {
            action,
            template_key,
            outpath,
            filename: String::from(filename),
        })
    }
}
