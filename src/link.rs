use std::{ fmt, path::PathBuf };

#[derive(Debug, Clone)]
pub struct LinkState {
    /// 存储三元组：(目标文件名, 符号链接路径, 链接状态)
    data: Vec<(String, PathBuf, bool)>,
}

impl LinkState {
    pub fn from_vec(data: Vec<(String, PathBuf, bool)>) -> Self {
        LinkState { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    pub fn valid_count(&self) -> usize {
        self.data.iter().filter(|&&(_, _, valid)| valid).count()
    }
    
    /// 获取无效的链接数量
    pub fn invalid_count(&self) -> usize {
        self.data.iter().filter(|&&(_, _, valid)| !valid).count()
    }
}

impl fmt::Display for LinkState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() {
            return writeln!(f, "No symbolic links found.");
        }
        
        // 计算各列的最大宽度
        let (max_target_len, max_path_len) = self.data.iter().fold(
            (0, 0),
            |(max_target, max_path), (target, path, _)| {
                (
                    max_target.max(target.chars().count()),
                    max_path.max(path.to_string_lossy().chars().count()),
                )
            }
        );
        
        // 确保最小列宽
        let target_width = max_target_len.max(10); // "Target File" 的长度
        let path_width = max_path_len.max(12);     // "Symbolic Link" 的长度
        let status_width = 10;                     // "Status" 的长度
        
        // 打印表头
        let total_width = target_width + path_width + status_width + 9; // 9 是边框和分隔符的宽度
        writeln!(f, "{}", "=".repeat(total_width))?;
        writeln!(
            f,
            "| {:<target_width$} | {:<path_width$} | {:^status_width$} |",
            "Target File", "Symbolic Link", "Status",
            target_width = target_width,
            path_width = path_width,
            status_width = status_width
        )?;
        writeln!(f, "|{}|{}|{}|", 
            "-".repeat(target_width + 2),
            "-".repeat(path_width + 2),
            "-".repeat(status_width + 2)
        )?;
        
        // 打印每一行数据
        for (target, path, is_valid) in &self.data {
            // 格式化状态显示
            let status = if *is_valid {
                "✓ VALID".to_string()
            } else {
                "✗ BROKEN".to_string()
            };
            
            // 格式化颜色（如果终端支持）
            let status_display = if *is_valid {
                format!("{}", status) // 可以在这里添加颜色代码，如 "\x1b[32m{}\x1b[0m"
            } else {
                format!("{}", status) // 可以在这里添加颜色代码，如 "\x1b[31m{}\x1b[0m"
            };
            
            writeln!(
                f,
                "| {:<target_width$} | {:<path_width$} | {:^status_width$} |",
                target,
                path.to_string_lossy(),
                status_display,
                target_width = target_width,
                path_width = path_width,
                status_width = status_width
            )?;
        }

        // 打印表尾和统计信息
        writeln!(f, "{}", "=".repeat(total_width))?;
        writeln!(
            f,
            "Total: {} links ({} valid, {} broken)",
            self.len(),
            self.valid_count(),
            self.invalid_count()
        )?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    
    use super::LinkState;

    /// ```
    /// cargo test output -- --nocapture
    /// ```
    #[test]
    fn output() {
        // 或者从已有数据创建
        let data = vec![
            ("file1.txt".to_string(), PathBuf::from("/link1"), true),
            ("file2.txt".to_string(), PathBuf::from("/link2"), false),
        ];
        let links = LinkState::from_vec(data);
        println!("{}", links);
    }
}