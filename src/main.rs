use std::{env, fs};
use std::path::PathBuf;
use std::io::prelude::*;
use structopt::StructOpt;

mod local;
mod remote;

#[derive(Debug, StructOpt)]
#[structopt(about = "the tool for managing your settings")]
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
    env_logger::init();

    let os_type = env::consts::OS;
    log::debug!("OS: {}", os_type);

    let opt = Once::from_args();
    log::debug!("command: {:?}", opt);

    match opt {
        Once::Init { root } => local::init(root),
        Once::New { programs } => local::new(&programs),
        Once::Check { programs } => local::check(&programs),
        Once::Link { programs } => local::link(&programs),
        Once::Unlink { programs } => local::unlink(&programs),
        Once::Install { programs } => local::install(&programs),
        Once::Immigrate { programs } => local::migrate(&programs),
        Once::Search { programs } => remote::search(&programs),
        Once::Import { programs } => remote::import_remote(&programs),
    };
}

fn get_config_path() -> PathBuf {
    let home_path = dirs::home_dir().unwrap();
    let mut config_path = home_path.clone();
    config_path.push(".config");
    config_path.push("once");

    config_path
}

fn get_root_path() -> PathBuf {
    let config_path = get_config_path();

    let mut file = fs::File::open(&config_path).expect("couldn't open config file: ~/.config/once");
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s.into()
}