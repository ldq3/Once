use std::{env, fs, path};
use std::path::PathBuf;
use local::init;
use std::io::prelude::*;
use structopt::StructOpt;

mod local;
mod remote;

#[derive(Debug, StructOpt)]
#[structopt(about = "the stupid content tracker")]
enum Once {
    Init {
        #[structopt(parse(from_os_str))]
        root: PathBuf,
    },
    New {
        programs: Vec<String>,
    },
    Check {
        programs: Vec<String>,
    },
    Link {
        programs: Vec<String>,
    },
    Unlink {
        programs: Vec<String>,
    },
    Install {
        programs: Vec<String>,
    },
    Immigrate {
        programs: Vec<String>,
    },
    Search {
        programs: Vec<String>,
    },
    Import {
        programs: Vec<String>,
    }
}

fn main() {
    let opt = Once::from_args();
    println!("{:?}", opt);

    let os_type = env::consts::OS;
    println!("{}", os_type);

    if let Once::Init { root } = opt {
        init(root)
    } else {
        let root = {
            let config_path = path::Path::new("~/.config/once");
            let display = config_path.display();
    
            let mut file = match fs::File::open(config_path) {
                Err(why) => panic!("couldn't open {}: {:?}", display, why),
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {:?}", display, why),
                Ok(_) => print!("{} contains:\n{}", display, s),
            }
    
            s
        };
    
        match opt {
            Once::New { programs } => local::new(&programs),
            Once::Check { programs } => local::check(&programs),
            Once::Link { programs } => local::link(&programs),
            Once::Unlink { programs } => local::unlink(&programs),
            Once::Install { programs } => local::install(&programs),
            Once::Immigrate { programs } => local::migrate(&programs),
            Once::Search { programs } => remote::search(&programs),
            Once::Import { programs } => remote::import_remote(&programs),
            _ => help(),
        };
    };
}

fn help() {
    println!("May you need help?")
}