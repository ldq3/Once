use std::{
    fs, io::ErrorKind, path::{self, PathBuf}
};
#[cfg(target_os = "linux")]
use std::os::unix::fs as os_fs;
#[cfg(target_os = "windows")]
use std::os::windows::fs as os_fs;
use std::io::{self, Write};
use toml::Table;
use serde::Deserialize;
use once;

use crate::get_root_path;
// use std::process::Command;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Once {
    windows: Config,
    linux: Config,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Config {
    commands: String,
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
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                
                input = input.trim().to_lowercase().to_string(); // 清除输入字符串末尾的换行符

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
    let root_path = path::PathBuf::from(root_str);
    let root_path = once::replace_home(root_path);
    
    if root_path.exists() && fs::metadata(&root_path).map_or(false, |md| md.is_dir()) {
        println!("Initializing once root:{:?}", &root_path);
        match file.write_all(root_path.as_os_str().as_encoded_bytes()) {
            Err(why) => panic!("couldn't write to {}: {:?}", config_path.display(), why),
            Ok(_) => println!("successfully write to {}", config_path.display()),
        }
    } else {
        panic!("Invalid root path: {}", root_str);
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

        #[cfg(target_os = "windows")]
        let contents = contents.replace("\r\n", "\n");

        let value: Once = toml::from_str(contents.as_str()).unwrap();

        #[cfg(target_os = "windows")]
        let links_iter = value.windows.links.iter();
    
        #[cfg(target_os = "linux")]
        let links_iter = value.linux.links.iter();

        for (key, value) in links_iter {
            let mut original = PathBuf::new();
            original.push(root.clone());
            original.push(program);
            original.push("settings");
            original.push(key);

            let link = path::PathBuf::from(value.as_str().unwrap());

            let link = once::replace_home(link);
            println!("{:?}, {:?}", original, link);
            
            if !original.exists() {
                println!("{} doesn't exist", original.display());
                continue;
            }

            if link.exists() {
                println!("{} exists", link.display());
                continue;
            }
            
            #[cfg(target_os = "linux")]
            os_fs::symlink(original, link).expect("Something wrong");

            #[cfg(target_os = "windows")]
            if fs::metadata(&original).map_or(false, |md| md.is_dir()) {
                os_fs::symlink_dir(original, link).expect("Something wrong");
            } else if fs::metadata(&original).map_or(false, |md| md.is_file()){
                os_fs::symlink_file(original, link).expect("Something wrong");
            }
        }
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

        #[cfg(target_os = "windows")]
        let contents = contents.replace("\r\n", "\n");

        let value: Once = toml::from_str(contents.as_str()).unwrap();
        
        #[cfg(target_os = "windows")]
        let links_iter = value.windows.links.iter();

        #[cfg(target_os = "linux")]
        let links_iter = value.linux.links.iter();

        for (_, link) in links_iter {
            let link = path::PathBuf::from(link.as_str().unwrap());

            let link = once::replace_home(link);
            
            match fs::remove_file(&link) {
                Err(why) if why.kind() == ErrorKind::NotFound => {
                    println!("link {} doesn't exit", link.display());
                    continue;
                },
                Err(why) => panic!("couldn't remove {}: {:?}", link.display(), why),
                Ok(_) => println!("successfully remove {}", link.display()),
            };
        }       
    }
}

pub fn install(programs: &[String]) {
    println!("This is install! I receive {:?}", programs)
}

pub fn migrate(programs: &[String]) {
    println!("This is migrate! I receive {:?}", programs)
}