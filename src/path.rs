use std::{ env::{ self, VarError }, fs::{self, File}, io::{self, ErrorKind, Read, Write}, path::PathBuf, collections::HashMap };
use shellexpand::{ tilde_with_context, env_with_context };
use toml;
use serde::{Deserialize, Serialize};

pub struct Conf {}

impl Conf {
    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap() // Windows and Linux
            .join("once")
    }

    pub fn init(root: String) -> Result<(), io::Error> {
        let path = Self::path();
        let parent_dirs = path
            .parent()
            .ok_or(io::Error::from(ErrorKind::InvalidInput))?;

        fs::create_dir_all(&parent_dirs)?;
        let mut file = File::create(path)?;
        file.write_all(root.as_str().as_bytes())?;

        Ok(())
    }

    pub fn root_dir() -> Result<PathBuf, io::Error> {
        let path = Self::path(); 

        let root_dir = fs::read_to_string(&path)?
            .trim_end()
            .to_string();

        let root_dir = parse_inner(&root_dir);

        Ok(root_dir)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Regstry {
    pub programs: Vec<String>,
}

impl Regstry {
    fn path() -> Result<PathBuf, io::Error> {
        let root_dir = Conf::root_dir()?;

        let path = root_dir
            .join("reg.toml");

        Ok(path)
    }

    fn get() -> Result<Self, io::Error> {
        let path = Self::path()?;

        let content = fs::read_to_string(&path)?;
        
        let reg = match toml::from_str(&content) {
            Ok(reg) => reg,
            _ => { unreachable!(); }
        };
        
        Ok(reg)
    }

    fn save(&self) -> Result<(), io::Error> {
        let toml_str = match toml::to_string_pretty(self) {
            Ok(toml_str) => toml_str,
            _ => unreachable!(),
        };

        let path = Self::path()?;
        fs::write(&path, toml_str)
    }

    pub fn init(root_dir: PathBuf) -> Result<(), io::Error> {
        let path = root_dir.join("reg.toml");
        let parent_dirs = path
            .parent()
            .ok_or(io::Error::from(ErrorKind::InvalidInput))?;

        fs::create_dir_all(&parent_dirs)?;
        File::create(path)?;

        Ok(())
    }

    /// add item to registry
    pub fn add(program: String) -> Result<(), io::Error> {
        let path = Self::path()?;
        let content = fs::read_to_string(&path)?;
    
        let mut reg: Regstry = match toml::from_str(&content) {
            Ok(reg) => reg,
            _ => unreachable!(),
        };    

        if reg.programs.contains(&program) { return Ok(()) };

        reg.programs.push(program);

        let toml_str = match toml::to_string_pretty(&reg) {
            Ok(toml_str) => toml_str,
            _ => unreachable!(),
        };

        fs::write(&path, toml_str)
    }
}

pub struct Program {
    name: String,
    links: HashMap<String, String>, 
}

impl Program {
    fn dir(name: &String) -> Result<PathBuf, io::Error> {
        let root_dir = Conf::root_dir()?;

        let dir = root_dir.join(name);

        Ok(dir)
    }

    fn record_path(name: &String) -> Result<PathBuf, io::Error> {
        let path = Self::dir(name)?
            .join("once.toml");

        Ok(path)
    }

    fn settings_dir(&self) -> Result<PathBuf, io::Error> {
        let dir = Self::dir(&self.name)?
            .join("settings");

        Ok(dir)
    }

    pub fn get(name: &String) -> Result<Self, io::Error> {
        let path = Self::record_path(name)?;

        let content = fs::read_to_string(path)?;
        let links: HashMap<String, String> = match toml::from_str(&content) {
            Ok(links) => links,
            _ => unreachable!(),
        };

        Ok(Program {
            name: name.clone(),
            links
        })
    }

    fn save(&self) -> Result<(), io::Error> {
        let toml_str = match toml::to_string_pretty(&self.links) {
            Ok(toml_str) => toml_str,
            _ => unreachable!(),
        };

        let path = Self::record_path(&self.name)?;
        fs::write(&path, toml_str)
    }

    pub fn init() {
        todo!()
    }

    pub fn add(&mut self, file: &String, link: &String) -> Result<(), io::Error> {
        todo!()
    }
}

/**
返回绝对路径
*/
pub fn parse(path: &str) -> Result<PathBuf, VarError> {
    #[cfg(target_os = "windows")]
    let path = path.replace('/', "\\");

    // 扩展 ~ 
    let expanded = tilde_with_context(&path, || {
        dirs::home_dir().map(|p| p.to_string_lossy().into_owned())
    });
 
    // 扩展环境变量
    let context = |var: &str| -> Result<Option<String>, VarError> {
        env::var(var)
            .map(Some)
    };
    let expanded = env_with_context(&expanded, context)
        .map_err(|e| { e.cause })?;
    
    Ok(PathBuf::from(expanded.into_owned()))
}

/// 读取中间文件
fn parse_inner(path: &str) -> PathBuf {
    match parse(path) {
        Ok(path) => path,
        _ => unreachable!()
    }
}