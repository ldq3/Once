mod local;
mod print_tab;

use std::path::PathBuf;
use structopt::StructOpt;
use print_tab::print_tab;

#[derive(Debug, StructOpt)]
#[structopt(about = "a tool for managing your settings")]
enum Command {
    Init {
        path: String,
    },
    Move {
        path: String,
    },
    New {
        program: String,
        file: String,
        link: String,
    },
    Edit {
        program: String,
        file: String,
        link: String,
    },
    Check,
    List {
        program: String,
    },
}

fn main() {
    env_logger::init();

    match Command::from_args() {
        Command::Init { path } => {
            local::init(&path).unwrap();
        },
        Command::Move { path } => {
            local::mov(&path).unwrap() 
        },
        Command::New { program, file, link } => {
            local::new(&program, &file, &link).unwrap();
        },
        Command::Edit { program, file, link } => {
            local::edit(&program, &file, &link).unwrap();
        },
        Command::Check => {
            let programs = local::get_programs().unwrap();
            let mut status = Vec::new();
            for program in programs {
                let mut state = true;
                let data = local::list(&program).unwrap();

                for (_, _, b) in data {
                    state &= b;
                    if !state { break; }
                }

                status.push(vec![program, state.to_string()]);
            }

            let header = vec!["Program".to_string(), "State".to_string()];
            print_tab(&header, &status);
        },
        Command::List { program } => {
            let data = local::list(&program).unwrap();
            let header = vec!["Target File", "Symlink", "State"];
            let link_state = link_state(&data);
            print_tab(&header, &link_state);
        },
    };
}

fn link_state(items: &[(String, PathBuf, bool)]) -> Vec<Vec<&str>> {
    items.iter()
        .map(|(name, path, flag)| {
            vec![
                name.as_str(),
                path.to_str().unwrap(),
                if *flag {
                    "✅ Ok"
                } else {
                    "❌ Broken"
                },
            ]
        })
        .collect()
}

