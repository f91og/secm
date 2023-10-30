// r# 用于创建原始字符串字面值（raw string literals）的标记。原始字符串字面值是一种特殊类型的字符串字面值，它允许你包含任意字符，包括特殊字符、换行符等，而不需要进行转义
const CRATES_HELP: &str = r#"
a                       cargo add
"#;

// #[derive(PartialEq, Clone, Eq, Hash, Copy)] 是一个宏属性（derive attribute），它用于自动生成一些常见的 Rust 特性（traits）的实现
// PartialEq 和 Eq 用于生成实现相等性比较（equality comparison）的方法，如 == 和 !=。这使得你可以比较 PanelName 枚举的实例，检查它们是否相等。
// Clone 用于生成实现克隆（cloning）的方法，这意味着你可以复制 PanelName 枚举的实例。
// Hash 用于生成实现哈希计算的方法，这对于在哈希表中存储 PanelName 枚举的实例很有用。
// Copy 用于生成实现按位复制（bitwise copy）的方法。这表示 PanelName 枚举的实例在赋值时不会导致所有权转移，而是会复制。
#[derive(PartialEq, Clone, Eq, Hash, Copy)]
pub enum PanelName {
    Filter,
    Secrets,
}

pub struct Panel {
    pub index: usize,
    pub panel_name: PanelName,
    pub content: Vec<String>,
}

// 方法使用 self 参数，关联函数不使用 self 参数
impl Panel {
    pub fn get_help(&self) -> String {
        // 解构数组，中括号 [name, page]: [&str; 2] 的部分用于声明并解构数组，2 是数组的长度。
        let [name, page]: [&str; 2] = match self.panel_name {
            PanelName::Filter => ["filter", CRATES_HELP],
            PanelName::Secrets => ["secrets", CRATES_HELP],
        };
        format!("This is a help page for `{}` module!\n{}", name, page)
    }
}