use std::path::{
    Component,
    PathBuf
};
use dirs;
use std::env;

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

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, result};

    use super::*;

    #[test]
    fn it_works() {
        let test_path = PathBuf::from("~/test");
        let result = replace_home(test_path);
        assert_eq!(result.as_os_str().to_str().unwrap(), "/home/leap/test");
    }
}