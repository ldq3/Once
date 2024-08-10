use std::{
    path::{
        Component,
        PathBuf
    },
    env,
};
use dirs;

pub fn replace_home(path: PathBuf) -> PathBuf {
    let home_dir = dirs::home_dir().ok_or("can not reach home dir").unwrap();

    let mut components = path.components();

    if let Some(Component::Normal(first_element)) = components.next() {
        let first_element = first_element.to_str().ok_or("invalid path element").unwrap();
        
        if first_element == "~" {
            let mut new_path = home_dir.clone();
            
            for component in components {
                new_path.push(component.as_os_str());
            }
            
            return new_path;
        }
    }

    // 如果路径不是以"~"开头，直接返回原路径
    path
}

pub fn parse_env(path: PathBuf) -> PathBuf {
    let mut new_path = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Normal(component) => {
                let component_str = component.to_str().unwrap_or("");
                if component_str.starts_with('$') {
                    // 提取环境变量名
                    let var_name = &component_str[1..];
                    // 获取环境变量的值
                    let var_value = env::var(var_name).unwrap_or_else(|_| {
                        eprintln!("无法解析环境变量: {}", var_name);
                        std::process::exit(1);
                    });
                    new_path.push(var_value);
                } else {
                    new_path.push(component);
                }
            }
            _ => {
                // 处理其他类型的组件（如根目录、父目录等）
                new_path.push(component.as_os_str());
            }
        }
    }

    new_path
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn replace_home_0() {
        let test_path = PathBuf::from("~/test");
        let result = replace_home(test_path);
        assert_eq!(result.as_os_str().to_str().unwrap(), "C:\\Users\\34635\\test");
    }

    #[test]
    fn parse_env_0() {
        let test_path = PathBuf::from("$DriverData\\hi");
        let result = parse_env(test_path);
        assert_eq!(result.as_os_str().to_str().unwrap(), "C:\\Windows\\System32\\Drivers\\DriverData\\hi")
    }
}