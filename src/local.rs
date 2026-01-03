use std::{
    fs::{ self, File }, io::Write, path::PathBuf
};
#[cfg(target_os = "linux")]
use std::os::unix::fs as os_fs;
#[cfg(target_os = "windows")]
use std::os::windows::fs as os_fs;
use toml::{ Table, Value };
use anyhow::{ Context, Result };
use dirs;
use shellexpand::{tilde_with_context, env_with_context};

pub fn init(path: &String) -> Result<()> {
    let path = abs(path);
    let config_path = config_path();

    let mut file = File::create(&config_path).context(format!("创建配置文件失败：{}", config_path.display()))?;
    writeln!(file, "{}", path.display())?;

    Ok(())
}

/**
向
*/
pub fn new(program: &String, file: &String, link: &String) -> Result<()> { 
    let mut table =  get_table(program)?;
    
    let file_path = root_path()?
        .join(program)
        .join("settings")
        .join(file);
    let link_path = abs(link);
    symlink(&file_path, &link_path)?;

    table.insert(file.to_string(), toml::Value::String(link.clone()));
    fs::write(file, toml::to_string_pretty(&Value::Table(table))?)?;

    Ok(())
}

pub fn list(program: &String) -> Result<Vec<(String, PathBuf, bool)>> {
    let table = get_table(program)?;

    let mut data = Vec::new();

    for (file, value) in table { 
        let file_path = root_path()?
            .join(program)
            .join("settings")
            .join(&file);
        let link_path = value.to_string();
        let link_path = abs(link_path.trim_matches('"')); // to_string 会多出一队引号


        if !link_path.exists() {
            data.push((file, link_path.clone(), false));
            continue;
        }
        
        let metadata = fs::symlink_metadata(&link_path)
            .with_context(|| format!("无法获取文件元数据: {}", &link_path.display()))?;
        
        if !metadata.file_type().is_symlink() {
            data.push((file, link_path, false));
            continue;
        }
        // 获取符号链接指向的目标
        let target = fs::read_link(&link_path)
            .with_context(|| format!("无法读取符号链接: {}", &link_path.display()))?;
        // println!("target: {}, file_path: {}.", target.display(), file_path.display());

        data.push((file, link_path, target == file_path));
    }

    Ok(data)
}

/**
返回绝对路径
*/
fn abs(path: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    let path = path.replace('/', "\\");

    // 先扩展 ~ 
    let expanded = tilde_with_context(&path, || {
        dirs::home_dir().map(|p| p.to_string_lossy().into_owned())
    });
 
    // 再扩展环境变量
    let expanded = env_with_context(&expanded, |var: &str| -> Result<Option<String>, shellexpand::LookupError<std::env::VarError>> {
        std::env::var(var)
            .map(Some)
            .map_err(|e| shellexpand::LookupError {
                var_name: var.to_string(),
                cause: e,
            })
    }).unwrap_or_else(|_| expanded.clone());
    
    PathBuf::from(expanded.into_owned())
}

fn symlink(file: &PathBuf, link: &PathBuf) -> Result<()> {
    #[cfg(target_os = "linux")]
    os_fs::symlink(original, link).expect("Something wrong");

    #[cfg(target_os = "windows")]
    if fs::metadata(file).map_or(false, |md| md.is_dir()) {
        os_fs::symlink_dir(file, link)?;
    } else if fs::metadata(file).map_or(false, |md| md.is_file()){
        os_fs::symlink_file(file, link)?;
    }

    Ok(())
}

pub fn root_path() -> Result<PathBuf> {
    let path = config_path();

    let root = fs::read_to_string(&path)
        .context("读取 root 路径错误。")?
        .trim_end()
        .to_string();

    let root = abs(&root);

    Ok(PathBuf::from(&root))
}

/**

*/
fn config_path() -> PathBuf {
    let path = dirs::config_dir()
        .unwrap()
        .join("once");

    path
}

pub fn get_table(program: &String) -> Result<Table> {
    let path = root_path()
        .context("获取 root 路径失败。")?
        .join(program)
        .join("once.toml");

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .context("创建配置目录失败")?;
        }
    }

    let content = if path.exists() {
        fs::read_to_string(&path)
            .context(format!("读取配置文件失败: {}", path.display()))?
    } else {
        let default_content = "# 配置文档\n\n";
        fs::write(&path, default_content)
            .context(format!("创建配置文件失败: {}", path.display()))?;
        default_content.to_string()
    };
    
    let table = match toml::from_str::<Table>(&content) {
        Ok(table) => table,
        Err(e) => {
            // 记录解析错误，但继续使用空表
            eprintln!("警告: 配置文件解析失败 {}: {}", path.display(), e);
            Table::new()
        }
    };
    
    Ok(table)
}

pub fn get_programs() -> Result<Vec<String>> {
    let mut dirs = Vec::new();
    let path = root_path()?;
    
    for entry in fs::read_dir(path.clone()).context(format!("root path: {}.", path.display()))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                dirs.push(dir_name.to_string_lossy().to_string());
            }
        }
    }
    
    Ok(dirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_0() {
        let result = abs("~/AppData/Roaming/Code/User/snippet");
        assert_eq!(result.as_os_str().to_str().unwrap(), "C:/Users/34635/AppData/Roaming/Code/User/snippet");
    }
}