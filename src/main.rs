// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::{Model, SharedString};

slint::include_modules!();

fn has_won(player_symbol: SharedString, board: &Vec<SharedString>) -> bool {
    // row
    for i in 0..3 {
        if board[3*i] == player_symbol && board[3*i + 1] == player_symbol && board[3*i + 2] == player_symbol {
            return true;
        }
    }
    //column
    if board[0] == player_symbol && board[3] == player_symbol && board[6] == player_symbol {
        return true;
    }
    if board[1] == player_symbol && board[4] == player_symbol && board[7] == player_symbol {
        return true;
    }
    if board[2] == player_symbol && board[5] == player_symbol && board[8] == player_symbol {
        return true;
    }
    // diagonal
    if board[0] == player_symbol && board[4] == player_symbol && board[8] == player_symbol {
        return true;
    }
    if board[2] == player_symbol && board[4] == player_symbol && board[6] == player_symbol {
        return true;
    }
    return false
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let win_dialog = WinWindow::new()?;

    let ui_handle = ui.as_weak();
    let win_handle = win_dialog.as_weak();
    ui.on_move(move |index| {
        let ui = ui_handle.upgrade().unwrap();
        let win_dialog = win_handle.upgrade().unwrap();
        let idx = index as usize;
        let mut fields: Vec<SharedString> = ui.get_fields().iter().collect();
        let current_player = ui.get_current_player();

        if fields[idx] == " " {
            fields[index as usize] = current_player.clone();

            if has_won(current_player.clone(), &fields) {

                win_dialog.set_message(SharedString::from(format!("Player {} has won!", current_player)));

                win_dialog.show();
            }


            if current_player == "X" {
                ui.set_current_player(SharedString::from("O"));
            } else {
                ui.set_current_player(SharedString::from("X"));
            }
        }
        ui.set_fields(std::rc::Rc::new(slint::VecModel::from(fields)).clone().into());
    });

    let ui_handle = ui.as_weak();
    let win_handle = win_dialog.as_weak();
    win_dialog.on_restart(move || {
        let ui = ui_handle.upgrade().unwrap();
        let win_dialog = win_handle.upgrade().unwrap();
        ui.set_fields(std::rc::Rc::new(slint::VecModel::from(vec![SharedString::from(" "); 9])).clone().into());
        ui.set_current_player(SharedString::from("X"));
        win_dialog.hide();
    });

    win_dialog.on_quit(move || {
        std::process::exit(0); // Exit the application
    });

    Ok(ui.run()?)
}
