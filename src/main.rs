use std::process;
use temp::{run, Config};
mod toml_store;
use std::env::args;
use std::path::PathBuf;

fn main() {
    let key = String::from("css");
    let path = PathBuf::from("temp/check");

    let store = toml_store::TemplateStore::new();

    println!("{:?}", store.store);

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
