use crate::games::puzzle_game::Puzzle16;
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
pub extern "C" fn puzzle_snapshot(puz: *const Puzzle16, out: *mut i32) -> usize {
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
