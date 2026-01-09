mod local;
mod link;

use std::{ fs, io };
use structopt::StructOpt;
use local::get_table;
use link::LinkState;

#[derive(Debug, StructOpt)]
#[structopt(about = "a tool for managing your settings")]
enum Command {
    Init {
        path: String,
    },
    New {
        program: String,
        file: String,
        link: String,
    },
    List {
        opt: Option<String>,
    },
}

fn main() {
    env_logger::init();

    match Command::from_args() {
        Command::Init { path } => {
            match local::root_path() {
                Ok(root) => {
                    let root = root.to_str().unwrap().to_string();
                    
                    if root != path {
                        print!("❓ root 已存在：{}，是否覆盖？(y/any key else):", root);

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        
                        let input = input.trim().to_lowercase();
                        if input == "y" {
                        } else {
                            println!("🚫 操作已取消");
                            
                            return;
                        }
                        fs::rename(&root, &path)
                            .expect(&format!("移动文件失败，from: {:?}, to: {:?}。", root, path));

                        println!("root 初始化成功");

                        return;
                    }
                },
                Err(e) => {
                    let io_err = e.downcast_ref::<std::io::Error>().unwrap();
                    if io_err.kind() != std::io::ErrorKind::NotFound {
                        panic!("{}", e);
                    }
                }
            }

            local::init(&path).unwrap();
        },
        Command::New { program, file, link } => {
            let table =  get_table(&program).unwrap();
            if let Some(value) = table.get(&file) {
                println!("❓ 链接已存在：{}，是否覆盖？(y/any key else):", value);
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                let input = input.trim().to_lowercase();
                if input == "y" {
                } else {
                    println!("🚫 操作已取消");
                    
                    return;
                }
            }
            local::new(&program, &file, &link).unwrap();
        },
        Command::List { opt } => {
            if let Some(program) = opt {
                let data = local::list(&program).unwrap();
                let link_state = LinkState::from_vec(data);
                println!("{}", link_state);
                
            } else {
                let programs = local::get_programs().unwrap();
                let mut status = Vec::new();
                for program in programs {
                    let mut state = true;
                    let data = local::list(&program).unwrap();

                    for (_, _, b) in data {
                        state &= b;
                        if !state { break; }
                    }

                    status.push(state);
                }
            }
        },
    };
}
