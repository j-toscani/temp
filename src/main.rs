use std::fs;
use std::path;

fn main() {
    let action = "create";
    let outpath = "";
    let filename = "hello.txt";
    let content = "Hello World!";

    let args = vec![action, outpath, filename];
    let out = path::PathBuf::from_iter([args[1], args[2]]);
    fs::write(out.as_path(), content).expect("Could not write File");
}
