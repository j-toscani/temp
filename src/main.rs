use std::fs;
use std::path;

fn main() {
    let action = "create";
    let outpath = "components";
    let filename = "hello.txt";
    let content = "Hello World!";

    let args = vec![action, outpath, filename];
    let out = path::PathBuf::from_iter([args[1], args[2]]);

    let parent = match out.parent() {
        Some(val) => val,
        None => path::Path::new("/"),
    };

    if !parent.exists() {
        match fs::create_dir_all(parent) {
            Ok(()) => (),
            Err(_) => panic!("Directory could not be created"),
        };
    }

    fs::write(out.as_path(), content).expect("Could not write File.");
}
