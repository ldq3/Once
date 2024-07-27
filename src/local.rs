use std::{
    fs,
    path::{self, PathBuf},
    os::unix::fs as ufs,
};
use dirs;
use std::io::{self, Write};
use toml::Table;
use std::io::prelude::*;
use serde::Deserialize;
use once_lib;
// use std::process::Command;

#[derive(Deserialize)]
struct Once {
    windows: Config,
    linux: Config,
}

#[derive(Deserialize)]
struct Config {
    commands: Vec<String>,
    links: Table
}

pub fn init(root: path::PathBuf) {
    let config_path = crate::get_config_path();

    let mut file = match fs::File::create_new(&config_path) {
        Err(reason) => {
            if reason.kind() == std::io::ErrorKind::AlreadyExists {
                println!("The config file {} already exists. Do you want to override it? (Y/N)", config_path.display());
                // 读取用户输入
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                // 清除输入字符串末尾的换行符
                input = input.trim().to_lowercase().to_string();
                // 根据用户输入决定是否覆盖文件
                if input == "y" {
                    fs::File::create(&config_path).expect("Failed to create file")
                } else {
                    panic!("Exiting without overriding the file.");
                }
            } else {
                panic!("Failed to create file: {}", reason);
            }
        },
        Ok(file) => file,
    };

    let root_str = root.to_str().expect("Path is not valid UTF-8");
    
    println!("Initializing once root:{}", &root_str);
    match file.write_all(root_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {:?}", config_path.display(), why),
        Ok(_) => println!("successfully write to {}", config_path.display()),
    }
}

pub fn new(programs: &[String]) {
    println!("This is new! I receive {:?}", programs)
}

pub fn check(programs: &[String]) {
    println!("This is check! I receive {:?}", programs)
}

pub fn link(programs: &[String]) {
    for program in programs.iter() {
        let mut program_config = PathBuf::new();
        program_config.push(program.clone());
        program_config.push("once.toml");

        println!("{:?}", program_config);
        let contents = fs::read_to_string(program_config)
        .expect("Something went wrong reading the file");

        let value: Once = toml::from_str(contents.as_str()).unwrap();

        for (key, value) in value.linux.links.iter() {
            let mut original = PathBuf::new();
            original.push(program);
            original.push("settings");
            original.push(key);

            let link = path::PathBuf::from(value.as_str().unwrap());

            let link = once_lib::replace_home(link);
            println!("{:?}, {:?}", original, link);
            ufs::symlink(original, link).expect("Something wrong");
        }
        
        // std::os::unix::fs::symlink and std::os::windows::fs::{symlink_file, symlink_dir}
    }
}

pub fn unlink(programs: &[String]) {
    for program in programs.iter() {
        let mut program_config = PathBuf::new();
        program_config.push(program.clone());
        program_config.push("once.toml");

        println!("{:?}", program_config);
        let contents = fs::read_to_string(program_config)
        .expect("Something went wrong reading the file");

        let value: Once = toml::from_str(contents.as_str()).unwrap();

        for (_, value) in value.linux.links.iter() {
            let link = path::PathBuf::from(value.as_str().unwrap());

            let link = once_lib::replace_home(link);
            println!("{:?}", link);
            fs::remove_file(link).expect("link doesn't exist")
        }
    }
}

pub fn install(programs: &[String]) {
    println!("This is install! I receive {:?}", programs)
}

pub fn migrate(programs: &[String]) {
    println!("This is migrate! I receive {:?}", programs)
}