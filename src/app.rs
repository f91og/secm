// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段
// 不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现
pub struct App {
    /// Current value of the input box
    pub input: String,
    pub secrets: Vec<String>, // 存放一些数据或者 UI 状态
    pub len_after_filtered: usize, // 过滤后的数据长度
    pub selected_secret_index: usize, // 选中的索引
}

// 为结构体添加方法
// impl App {
//     pub fn new() -> App {
//         App {
//             secrets: vec![],
//         }
//     }
// }