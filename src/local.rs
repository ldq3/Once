use std::{
    fs,
    path::{self, PathBuf},
};
use std::io::{self, Write};
use toml::Table;
use std::io::prelude::*;
// use std::process::Command;

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