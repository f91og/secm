use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use secm::app::App;
use secm::ui;

fn main() -> Result<(), io::Error> {
    // 1.初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // secret list got from file
    let secrets = vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string(),];

    let app = App {
        input: String::new(),
        secrets: secrets,
    };

    // 2.渲染界面
    run_app(&mut terminal, app)?;

    // 3.恢复终端
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
        terminal.draw(|f| ui::ui(f, &mut app))?;    // 把绘制界面逻辑放到ui模块中
        // 处理按键事件
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => {
                        // if 'q' == ch {
                        //     break;
                        // }
                        app.input.push(ch);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    },
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}