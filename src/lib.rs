use crate::games::{puzzle_game::Puzzle16, tic_tac_toe_game::TicTacToe};
pub mod games;

// PuzzleGame
#[unsafe(no_mangle)]
pub extern "C" fn puzzle_new() -> *mut Puzzle16 {
    let v = Puzzle16::new();
    Box::into_raw(Box::new(v))
}

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_action(puz: *mut Puzzle16, op: i32) {
    assert!(!puz.is_null());
    unsafe {
        (*puz).action(op);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_is_win(puz: *mut Puzzle16) -> bool {
    assert!(!puz.is_null());
    unsafe {
        return (*puz).iswin();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_snapshot(puz: *const Puzzle16, out: *mut i32) -> u32 {
    use core::ptr::copy_nonoverlapping;
    if puz.is_null() || out.is_null() {
        return 0;
    }
    unsafe {
        let src = (*puz).get_board_slice();
        copy_nonoverlapping(src.as_ptr(), out, 16);
    }
    16
}

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_free(puz: *mut Puzzle16) {
    if !puz.is_null() {
        unsafe {
            drop(Box::from_raw(puz));
        }
    }
}

// tictactoe
#[unsafe(no_mangle)]
pub extern "C" fn tictactoe_new() -> *mut TicTacToe {
    let game = TicTacToe::new(3);
    Box::into_raw(Box::new(game))
}

#[unsafe(no_mangle)]
pub extern "C" fn tictactoe_free(game: *mut TicTacToe) {
    if !game.is_null() {
        unsafe {
            drop(Box::from_raw(game));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tictactoe_player1_action(game: *mut TicTacToe, grid: u32) -> bool {
    assert!(!game.is_null());
    unsafe {
        match (*game).player1(grid) {
            Ok(iswin) => {
                return iswin;
            },
            Err(msg) => {
                println!("{}", msg);
                false
            }
        }

    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tictactoe_player2_action(game: *mut TicTacToe, grid: u32) -> bool {
    assert!(!game.is_null());
    unsafe {
        match (*game).player2(grid) {
            Ok(iswin) => {
                return iswin;
            },
            Err(msg) => {
                println!("{}", msg);
                false
            }
        }

    }
}