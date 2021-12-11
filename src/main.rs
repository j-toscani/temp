use temp::Config;

fn main() {
    let action = "create";
    let outpath = "components";
    let filename = "hello.txt";

    let args = vec![action, outpath, filename];

    Config::new($args);
    // let out = PathBuf::from_iter([args[1], args[2]]);

    // let parent = match out.parent() {
    //     Some(val) => val,
    //     None => Path::new("/"),
    // };

    // if !parent.exists() {
    //     let mut input = String::new();
    //     println!("Directory does not exist. Do you want it to be created? [Y/N]");
    //     let should_create = match io::stdin().read_line(&mut input) {
    //         Ok(_n) => input.trim().to_lowercase() == String::from("y"),
    //         Err(_) => false,
    //     };
    //     if should_create {
    //         match fs::create_dir_all(parent) {
    //             Ok(()) => (),
    //             Err(_) => panic!("Directory could not be created"),
    //         };
    //     } else {
    //         println!("Directory was not created.");
    //         return;
    //     }
    // }

    // fs::write(out.as_path(), content).expect("Could not write File.");
}
