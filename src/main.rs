use std::process;
use temp::{run, Config};
mod toml_store;
use std::env::args;

fn main() {
    let _store = toml_store::TemplateStore::new();

    // let args: Vec<String> = args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // run(config).unwrap_or_else(|err| {
    //     eprintln!("{}", err);
    //     process::exit(1);
    // });
}
