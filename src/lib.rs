use crate::puzzle::tools::Puzzle16;
pub mod puzzle;

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_new(_n: usize) -> *mut Puzzle16 {
    // 建立固定容量的 Vec，之後只改內容、不增刪，避免 reallocate
    let v = Puzzle16::new();
    Box::into_raw(Box::new(v))
}

// 範例：做一次「就地」更新（不改長度/容量）
#[unsafe(no_mangle)]
pub extern "C" fn action(puz: *mut Puzzle16, op: i32) {
    assert!(!puz.is_null());
    unsafe {
        (*puz).action(op);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn is_win(puz: *mut Puzzle16) -> bool {
    assert!(!puz.is_null());
    unsafe {
        return (*puz).iswin();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_free(puz: *mut Puzzle16) {
    if !puz.is_null() {
        unsafe { drop(Box::from_raw(puz)); } // 交回 Box，自動釋放 Vec
    }
}
