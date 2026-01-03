use std::{ fs, io };
use structopt::StructOpt;

mod local;

use local::get_table;

#[derive(Debug, StructOpt)]
#[structopt(about = "a tool for managing your settings")]
enum Once {
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

    let opt = Once::from_args();

    match opt {
        Once::Init { path } => {
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
        Once::New { program, file, link } => {
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
        Once::List { opt } => {
            if let Some(program) = opt {
                let data = local::list(&program).unwrap();

                println!("┌──────────┬────────────────────────────────────────────┬──────────────┬────────┐");
                println!("│  序号    │             名称                           │     路径     │  状态  │");
                println!("├──────────┼────────────────────────────────────────────┼──────────────┼────────┤");
                
                for (index, (name, path, flag)) in data.iter().enumerate() {
                    // 截断过长的名称和路径以保持表格整齐
                    let name_display = if name.len() > 30 {
                        format!("{}...", &name[..27])
                    } else {
                        name.clone()
                    };
                    
                    let path_str = path.display().to_string();
                    let path_display = if path_str.len() > 30 {
                        format!("{}...", &path_str[..27])
                    } else {
                        path_str
                    };
                    
                    let status = if *flag { "启用" } else { "禁用" };
                    let status_icon = if *flag { "✅" } else { "❌" };
                    
                    println!("│ {:8} │ {:42} │ {:12} │ {:<2} {:4} │", 
                        index + 1, 
                        name_display,
                        path_display,
                        status_icon,
                        status
                    );
                }
                
                println!("└──────────┴────────────────────────────────────────────┴──────────────┴────────┘");
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
