use std::time::Instant;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppErr, Mode};
use crate::panel::PanelName;
use crate::utils;

pub fn handle_key_in_filter_mode(app: &mut App, key: KeyEvent) {
    let panel = app.get_panel(PanelName::Filter);
    match key.code {
        KeyCode::Char(ch) => panel.content[0].push(ch),
        KeyCode::Backspace => _ = panel.content[0].pop(),
        KeyCode::Esc => app.switch_mode(Mode::Normal),
        KeyCode::Enter => app.mode = Mode::Normal,
        KeyCode::Down => app.select_next(),
        KeyCode::Up => app.select_previous(),
        _ => {}
    }
}

pub fn handle_key_in_make_mode(app: &mut App, key: KeyEvent) {
    let panel = app.get_panel(PanelName::MakeSecret);

    match key.code {
        KeyCode::Esc => app.switch_mode(Mode::Normal),
        KeyCode::Char(ch) => panel.content[panel.index].push(ch),
        KeyCode::Backspace => _ = panel.content[panel.index].pop(),
        KeyCode::Tab => panel.index = (panel.index + 1) % 3,
        KeyCode::Enter => {

            let length = panel.content[1].trim();
            let n = match length.parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    app.error = AppErr{msg: "Length must be number", error_timer: Some(Instant::now())};
                    return;
                }
            };

            let name = panel.content[0].trim().to_string();
            let advance = panel.content[2].trim();
            let value = utils::generate_random_string(n, advance == "yes" || advance == "y");

            if let Err(err) = app.add_secret(name, value) {
                app.error = AppErr{msg: &err, error_timer: Some(Instant::now())};
            } else {
                app.switch_mode(Mode::Normal)
            }
        }
        _ => {}
    }
}

pub fn handle_key_in_update_mode(app: &mut App, key: KeyEvent) {
    let panel = app.get_panel(PanelName::UpdateSecret);
    match key.code {
        KeyCode::Char(ch) => panel.content[panel.index].push(ch),
        KeyCode::Backspace => _ = panel.content[panel.index].pop(),
        KeyCode::Esc => app.switch_mode(Mode::Normal),
        KeyCode::Enter => {
            if let Err(err) = app.update_selected_secret() {
                app.error = AppErr{msg: &err, error_timer: Some(Instant::now())};
            } else {
                app.switch_mode(Mode::Normal)
            }
        },
        KeyCode::Tab => panel.index ^= 1,
        _ => {}
    }
}

pub fn handle_key_in_delete_mode(app: &mut App, key: KeyEvent) {
    let panel = app.get_panel(PanelName::DeleteSecret);
    match key.code {
        KeyCode::Char(ch) => panel.content[0].push(ch),
        KeyCode::Backspace => _ = panel.content[0].pop(),
        KeyCode::Esc => app.switch_mode(Mode::Normal),
        KeyCode::Enter => {
            if panel.content[0].trim() == "y" {
                if let Err(err) = app.delete_selected_secret() {
                    app.error = AppErr{msg: &err, error_timer: Some(Instant::now())};
                } else {
                    app.switch_mode(Mode::Normal)
                }
            } else {
                app.switch_mode(Mode::Normal)
            }
        },
        _ => {}
    }
}

pub fn handle_key_in_add_mode(app: &mut App, key: KeyEvent) {
    let panel = app.get_panel(PanelName::AddSecret);
    match key.code {
        KeyCode::Char(ch) => panel.content[panel.index].push(ch),
        KeyCode::Backspace => _ = panel.content[panel.index].pop(),
        KeyCode::Enter => {
            let name = panel.content[0].trim().to_string();
            let value = panel.content[1].trim().to_string();
            if let Err(err) = app.add_secret(name, value) {
                app.error = AppErr{msg: &err, error_timer: Some(Instant::now())};
            } else {
                app.switch_mode(Mode::Normal)
            }
        },
        KeyCode::Esc => app.switch_mode(Mode::Normal),
        KeyCode::Tab => panel.index ^= 1,
        _ => {}
    }
}

pub fn handle_key_in_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(ch) => {
            match ch {
                'q' => app.should_exit = true,
                'j' => app.select_next(),
                'k' => app.select_previous(),
                'r' => app.switch_mode(Mode::Update),
                'm' => app.switch_mode(Mode::Make),
                'a' => app.switch_mode(Mode::Add),
                '/' => app.switch_mode(Mode::Filter),
                'd' => app.switch_mode(Mode::Delete),
                _ => {}
            }
        }
        KeyCode::Down => app.select_next(),
        KeyCode::Up => app.select_previous(),
        KeyCode::Esc => app.should_exit = true,
        KeyCode::Enter => app.copy_selected_to_clipboard(),    // 复杂的处理放到keymaps里去
        _ => {}
    }
}