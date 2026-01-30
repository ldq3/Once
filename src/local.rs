use crate::path::{Conf, Program, Regstry, parse};
use std::{
    fs::{ self, File }, io::Write, path::PathBuf
};
#[cfg(target_os = "linux")]
use std::os::unix::fs as os_fs;
#[cfg(target_os = "windows")]
use std::os::windows::fs as os_fs;
use anyhow::{ Result, Context };
use toml::{ Table, Value };

pub fn init(root_dir: String) -> Result<()> {
    let path = parse(&root_dir)
        .context("错误的 root 路径。")?;
    
    Regstry::init(path)?;

    Conf::init(root_dir)?;

    Ok(())
}

pub fn mov(path: &String) -> Result<()> {
    let path = parse(&path)
        .context("错误的 root 路径。")?;
    let root = Conf::root_dir()?;

    if path != root {
        fs::rename(&root, &path)?;

        todo!() // 更新所有链接
    }

    Ok(())
}

/**
向
*/
pub fn new(name: &String, file: &String, link: &String) -> Result<()> { 
    let mut program =  Program::get(name)?;
    
    program.add(file, link);
    let link_path = parse(link)
        .context("错误的 link 路径。")?;
    symlink(&file_path, &link_path)?;

    table.insert(file.to_string(), Value::String(link.clone()));
    fs::write(file, toml::to_string_pretty(&Value::Table(table))?)?;

    Ok(())
}

pub fn edit(program: &String, file: &String, link: &String) -> Result<()> {
    let mut table = get_table(program)?;

    let file_path = root_path()?
        .join(program)
        .join("settings")
        .join(file);
    let link_path = abs(link);
    
    let path = table
        .get_mut(file)
        .ok_or_else(|| anyhow!("条目不存在"))?;
    fs::remove_file(PathBuf::from(path.to_string()))?;
    symlink(&file_path, &link_path)?;

    *path = Value::String(link.clone());

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
        
        let metadata = fs::symlink_metadata(&link_path)?;
        
        if !metadata.file_type().is_symlink() {
            data.push((file, link_path, false));
            continue;
        }
        // 获取符号链接指向的目标
        let target = fs::read_link(&link_path)?;
        // println!("target: {}, file_path: {}.", target.display(), file_path.display());

        data.push((file, link_path, target == file_path));
    }

    Ok(data)
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
