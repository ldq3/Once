use std::{
    fs,
    path::{self, PathBuf},
};
use dirs;
use toml::Table;
use std::io::prelude::*;
// use std::process::Command;

pub fn init(root: path::PathBuf) {
    let home_path = dirs::home_dir().expect("Can not reach home dir.");
    let mut config_path = home_path.clone();
    config_path.push(".config");
    config_path.push("once");

    let mut file = match fs::File::create_new(&config_path) {
        Err(why) => panic!("couldn't open {}: {:?}", config_path.display(), why),
        Ok(file) => file,
    };

    let root_str = root.to_str().expect("Path is not valid UTF-8");
    println!("root_str:{}", &root_str);

    match file.write_all(root_str.as_bytes()) {
        Err(why) => panic!("couldn't read {}: {:?}", config_path.display(), why),
        Ok(_) => println!("successfully wrote to {}", config_path.display()),
    }

    println!("This is init, I received {}", root.display());
}

pub fn new(programs: &[String]) {
    println!("This is new! I receive {:?}", programs)
}

pub fn check(programs: &[String]) {
    println!("This is check! I receive {:?}", programs)
}

pub fn link(programs: &[String]) {
    println!("This is link! I receive {:?}", programs);

    for program in programs.iter() {
        let contents = fs::read_to_string(program)
        .expect("Something went wrong reading the file");

        println!("With text:\n{}", contents);

        let value = contents.parse::<Table>().unwrap();

        println!("Hi! {:?}", value["package"]["name"]);

        // std::os::unix::fs::symlink and std::os::windows::fs::{symlink_file, symlink_dir}
    }
}

pub fn unlink(programs: &[String]) {
    println!("This is unlink! I receive {:?}", programs)
}

pub fn install(programs: &[String]) {
    println!("This is install! I receive {:?}", programs)
}

pub fn migrate(programs: &[String]) {
    println!("This is migrate! I receive {:?}", programs)
}