use std::{
    fs,
    path::{self, PathBuf},
};
#[cfg(target_os = "linux")]
use std::os::unix::fs as os_fs;
#[cfg(target_os = "windows")]
use std::os::windows::fs as os_fs;
use dirs;
use std::io::{self, Write};
use toml::Table;
use std::io::prelude::*;
use serde::Deserialize;
use once_lib;

use crate::get_root_path;
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

#[derive(Deserialize)]
struct Program {
    files: Table,
    folders: Vec<String>,
}

const PROGRAM: &str = r#"
folders = ["settings", "states"]

[files]
"once.toml" = """
[links]
original = 'link'

[windows]
commands = '''
Write-Host "Hello Once!"
'''

[windows.links]
original = 'link'

[linux]
commands = '''
echo "Hello Once!"
'''

[linux.links]
original = 'link'
"""
"#;

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
    let template: Program = toml::from_str(PROGRAM).unwrap();

    let root = get_root_path();

    for program in programs.iter() {
        let mut program_dir = root.clone();
        program_dir.push(program);
        fs::create_dir(&program_dir).unwrap(); 

        for (name, content) in template.files.iter() {
            let mut path = PathBuf::new();
            path.push(&program_dir);
            path.push(name);

            let parent = path.parent().unwrap();
            fs::create_dir_all(parent).unwrap();

            let content = content.as_str().unwrap();
            fs::write(path, content).unwrap();
        }

        for folder in template.folders.iter() {
            let mut path = PathBuf::new();
            path.push(&program_dir);
            path.push(folder);

            fs::create_dir(path).unwrap();
        }
    }
}

pub fn check(programs: &[String]) {
    println!("This is check! I receive {:?}", programs)
}

pub fn link(programs: &[String]) {
    let root = crate::get_root_path();
    
    for program in programs.iter() {
        let mut program_config = root.clone();
        program_config.push(program.clone());
        program_config.push("once.toml");

        println!("{:?}", program_config);
        let contents = fs::read_to_string(program_config)
        .expect("Something went wrong reading the file");

        let value: Once = toml::from_str(contents.as_str()).unwrap();

        for (key, value) in value.linux.links.iter() {
            let mut original = PathBuf::new();
            original.push(root.clone());
            original.push(program);
            original.push("settings");
            original.push(key);

            let link = path::PathBuf::from(value.as_str().unwrap());

            let link = once_lib::replace_home(link);
            println!("{:?}, {:?}", original, link);
            #[cfg(target_os = "linux")]
            os_fs::symlink(original, link).expect("Something wrong");
            #[cfg(target_os = "windows")]
            os_fs::symlink_file(original, link).expect("Something wrong");
        }
        
        // std::os::unix::fs::symlink and std::os::windows::fs::{symlink_file, symlink_dir}
    }
}

pub fn unlink(programs: &[String]) {
    for program in programs.iter() {
        let mut program_config = PathBuf::new();

        let root = get_root_path();

        program_config.push(root.clone());
        program_config.push(program.clone());
        program_config.push("once.toml");

        println!("{:?}", program_config);
        let contents = fs::read_to_string(program_config).unwrap();

        let value: Once = toml::from_str(contents.as_str()).unwrap();

        for (_, link) in value.linux.links.iter() {
            let link = path::PathBuf::from(link.as_str().unwrap());

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