use std::fmt::Display;
use unicode_width::UnicodeWidthStr;

/// 打印表格
/// 
/// # 参数
/// - header: 表头，每个元素是列名
/// - data: 表格数据，每个元素是一行数据
/// 
/// # 示例
/// ```
/// let header = vec!["姓名", "年龄", "城市"];
/// let data = vec![
///     vec!["张三", "25", "北京"],
///     vec!["李四", "30", "上海"],
///     vec!["王五", "28", "广州"],
/// ];
/// print_tab(&header, &data);
/// ```
pub fn print_tab<T: Display>(header: &[T], data: &[Vec<T>]) {
    let n = header.len();
    
    // 检查数据完整性
    if n == 0 {
        println!("⚠️  表头为空");
        return;
    }
    
    // 检查每行数据的列数是否与表头一致
    for (i, row) in data.iter().enumerate() {
        if row.len() != n {
            println!("⚠️  第 {} 行数据列数 ({}) 与表头列数 ({}) 不一致", 
                i + 1, row.len(), n);
            return;
        }
    }

    let max_item_width: usize = 20; // 增加最大宽度限制
    let board_width: usize = 3 * n + 1; // 修正边框宽度计算
    
    // 计算每列的最大宽度
    let mut lengths: Vec<usize> = vec![0; n];
    
    // 首先考虑表头宽度
    for (i, h) in header.iter().enumerate() {
        lengths[i] = h.to_string().width();
    }
    
    // 然后考虑数据中的宽度
    for row in data {
        for (i, d) in row.iter().enumerate() {
            lengths[i] = lengths[i].max(d.to_string().width());
        }
    }
    
    // 限制每列的最大宽度
    for length in &mut lengths {
        if *length > max_item_width {
            *length = max_item_width;
        }
    }
    
    // 计算表格总宽度（包括边框和分隔符）
    let total_width = lengths.iter().fold(
        board_width,
        |acc, &len| acc + len
    );

    // 构建表格字符串
    let mut s = String::new();
    
    // 顶部边框
    s.push_str(&format!("{}\n", "=".repeat(total_width)));
    
    // 表头行
    s.push_str("| ");
    for (i, h) in header.iter().enumerate() {
        let width = lengths[i];
        let header_str = h.to_string();
        
        // 如果表头过长，截断并添加省略号 #FIXME：中文字符的截断问题
        let display_str = if header_str.width() > width {
            format!("{}...", &header_str[..width.saturating_sub(3)])
        } else {
            header_str
        };
        
        s.push_str(&format!(
            "{:^width$} | ",
            display_str,
            width = width
        ));
    }
    s.push('\n');
    
    // 表头分隔线
    s.push_str("|");
    for width in &lengths {
        s.push_str(&format!("{}|", "-".repeat(width + 2)));
    }
    s.push('\n');
    
    // 数据行
    for row in data {
        s.push_str("| ");
        for (i, cell) in row.iter().enumerate() {
            let width = lengths[i];
            let cell_str = cell.to_string();
            
            // 如果单元格内容过长，截断并添加省略号
            let display_str = if cell_str.width() > width {
                format!("{}...", &cell_str[..width.saturating_sub(3)])
            } else {
                cell_str
            };
            
            s.push_str(&format!(
                "{:<width$} | ",
                display_str,
                width = width - display_str.width() + display_str.chars().count()
            ));
        }
        s.push('\n');
    }
    
    // 底部边框
    s.push_str(&format!("{}", "=".repeat(total_width)));
    
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_tab() {
        let header = vec!["ID", "name", "age", "city"];
        let data = vec![
            vec!["1", "张三", "25", "beijing"],
            vec!["2", "lisi", "30", "shanghai"],
            vec!["3", "安杰洛", "28", "guangzhou"],
        ];
        
        print_tab(&header, &data);
    }

    #[test]
    fn display_width() {
        assert_eq!("李".width(), 2);
        assert_eq!("李".len(), 3);
    }
}