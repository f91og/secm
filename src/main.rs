use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    terminal::Terminal,
};

use secm::app::App;
use secm::ui;
use secm::cmds;

const ERROR_MSG: &str = r#"
"Usage:
 - secm # enter secret management ui
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        if let Err(err) = scem() {
            println!("{}", err);
        }
        return;
    }
    if args.len() < 3  {
        panic!("{}", ERROR_MSG)
    }

    // let secret_file = &utils::get_secret_file_path();

    let verb = args[1].trim();
    match verb {
        "make" => {
            if let Err(err) = cmds::cmd_make(&args[2..]) {
                println!("{}", err);
            }
        },
        _ => {print!("{} {}", ERROR_MSG, verb);},
    }
}

fn scem() -> Result<(), Box<dyn Error>> {
    // 1.初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();

    // 2.渲染界面
    let res = run_app(&mut terminal, app);

    // 3.恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
        println!("{:?}", ERROR_MSG);
    }

    Ok(())
}

// 在 Rust 中，`?` 符号用于处理 `Result` 或 `Option` 类型的错误处理。当你在一个函数中使用 `?` 运算符时，它会尝试获取 `Result` 或 `Option` 类型的值，如果是 `Ok` 或 `Some`，那么它会解包这个值，否则，它会提早从函数中返回 `Err` 或 `None`。
// 在你提供的代码中，`terminal.draw(...)?` 行中的 `?` 用于处理 `Result` 类型的错误。具体来说，它会检查 `terminal.draw(...)` 的返回值，如果结果是 `Err`，那么它会提早从 `run_app` 函数返回该错误，使得调用 `run_app` 函数的地方可以进一步处理错误或者中止程序。如果结果是 `Ok`，那么程序会继续运行。
// 这种用法使得错误处理更加方便和紧凑，避免了显式的 `match` 或 `if let` 语句来处理每个可能的错误情况。如果使用 `?` 运算符，你可以将错误传播到调用者，以便在更高层次上进行处理。
// 在你的示例中，当 `crossterm::event::poll(Duration::from_secs(1))?` 或 `event::read()?` 出现错误时，程序将尽早返回错误，以确保错误得到适当的处理。
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    while !app.should_exit {
        terminal.draw(|f| ui::ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        };
    }
    Ok(())
}
