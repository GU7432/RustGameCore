use crate::puzzle::tools::Puzzle16;
pub mod puzzle;

#[unsafe(no_mangle)]
pub extern "C" fn puzzle_new(_n: usize) -> *mut Puzzle16 {
    let v = Puzzle16::new();
    Box::into_raw(Box::new(v))
}


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
