use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    // text::{Span, Spans, Text},
    Terminal,
    Frame,
};

// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段
// 不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现
struct App {
    items: Vec<String>, // 存放一些数据或者 UI 状态
}

fn main() -> Result<(), io::Error> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App {
        items: vec![
            "Item 1".to_string(), // &str -> String
            "Item 2".to_string(),
            "Item 3".to_string(),
            "Item 4".to_string(),
            "Item 5".to_string(),
            "Item 6".to_string(),
            "Item 7".to_string(),
            "Item 8".to_string(),
            "Item 9".to_string(),
            "Item 10".to_string(),
        ],
    };

    // 渲染界面
    run_app(&mut terminal, app)?;

    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

// 在 Rust 中，`?` 符号用于处理 `Result` 或 `Option` 类型的错误处理。当你在一个函数中使用 `?` 运算符时，它会尝试获取 `Result` 或 `Option` 类型的值，如果是 `Ok` 或 `Some`，那么它会解包这个值，否则，它会提早从函数中返回 `Err` 或 `None`。
// 在你提供的代码中，`terminal.draw(...)?` 行中的 `?` 用于处理 `Result` 类型的错误。具体来说，它会检查 `terminal.draw(...)` 的返回值，如果结果是 `Err`，那么它会提早从 `run_app` 函数返回该错误，使得调用 `run_app` 函数的地方可以进一步处理错误或者中止程序。如果结果是 `Ok`，那么程序会继续运行。
// 这种用法使得错误处理更加方便和紧凑，避免了显式的 `match` 或 `if let` 语句来处理每个可能的错误情况。如果使用 `?` 运算符，你可以将错误传播到调用者，以便在更高层次上进行处理。
// 在你的示例中，当 `crossterm::event::poll(Duration::from_secs(1))?` 或 `event::read()?` 出现错误时，程序将尽早返回错误，以确保错误得到适当的处理。
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // 处理按键事件
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => {
                        if 'q' == ch {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Render the filter input
    let filter_block = Block::default().borders(Borders::ALL);
    f.render_widget(filter_block, chunks[0]);

    // Render the list of items
    let items_block = Block::default().borders(Borders::ALL);

    let items: Vec<ListItem> = app.items.iter().map(|item| ListItem::new(item.clone())).collect();
    let items_list = List::new(items)
            .block(items_block)
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));
    f.render_widget(items_list, chunks[1]);

    // let chunks = Layout::default() // 首先获取默认构造
    //     .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
    //     .direction(Direction::Vertical) // 垂直分割
    //     .split(f.size()); // 分割整块 Terminal 区域
    // let paragraph = Paragraph::new(Span::styled(
    //     app.url.as_str(),
    //     Style::default().add_modifier(Modifier::BOLD),
    // ))
    // .block(Block::default().borders(Borders::ALL).title("HelloGitHub"))
    // .alignment(tui::layout::Alignment::Left);
    // f.render_widget(paragraph, chunks[0]);

    // let paragraph = Paragraph::new("分享 GitHub 上有趣、入门级的开源项目")
    //     .style(Style::default().bg(Color::White).fg(Color::Black))
    //     .block(Block::default().borders(Borders::ALL).title("宗旨"))
    //     .alignment(Alignment::Center);
    // f.render_widget(paragraph, chunks[1]);
}