use std::fs;
use toml::Table;
// use std::process::Command;

pub fn init() {
    println!("There is nothing!")
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