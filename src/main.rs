use std::{ env, error::Error, ffi::OsString, io, path::PathBuf };
use structopt::StructOpt;

mod local;

use local::get_table;

#[derive(Debug, StructOpt)]
#[structopt(about = "the tool for managing your settings")]
enum Once {
    Init {
        #[structopt(parse(from_os_str))]
        root: OsString,
    },
    New {
        program: String,
        file: String,
        #[structopt(parse(from_os_str))]
        link: PathBuf,
    },
    List {
        opt: Option<String>,
    },
}

fn main() {
    env_logger::init();

    let os_type = env::consts::OS;
    log::debug!("OS: {}", os_type);

    let opt = Once::from_args();
    log::debug!("command: {:?}", opt);

    match opt {
        Once::Init { root: path } => {
            local::init(path.to_str().unwrap()).unwrap();
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
