use std::{env, path};
use std::path::PathBuf;
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

    match opt {
        Once::Init { root } => println!("This is init, I received {}", root.display()),
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

fn help() {
    println!("May you need help?")
}