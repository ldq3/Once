use std::env;

mod local;
mod remote;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let command = &args[1];
    let programs = &args[2..];

    let os_type = env::consts::OS;
    println!("{}", os_type);

    match command.as_str() {
        "init" => local::init(),
        "new" => local::new(programs),
        "check" => local::check(programs),
        "link" => local::link(programs),
        "unlink" => local::unlink(programs),
        "install" => local::install(programs),
        "migrate" => local::migrate(programs),
        "search" => remote::search(programs),
        "import" => remote::import_remote(programs),
        _ => help()
    }
}

fn help() {
    println!("May you need help?")
}