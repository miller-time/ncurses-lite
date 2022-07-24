#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::CString;

use ffi::WINDOW;

pub enum CURSOR_VISIBILITY {
    CURSOR_INVISIBLE,
}

mod ffi;

pub fn initscr() -> WINDOW {
    unsafe { ffi::initscr() }
}

pub fn endwin() -> i32 {
    unsafe { ffi::endwin() }
}

pub fn curs_set(visibility: CURSOR_VISIBILITY) -> i32 {
    unsafe { ffi::curs_set(visibility as i32) }
}

pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW {
    unsafe { ffi::newwin(lines, cols, y, x) }
}

pub fn delwin(w: WINDOW) -> i32 {
    unsafe { ffi::delwin(w) }
}

pub fn waddch(w: WINDOW, ch: u32) -> i32 {
    unsafe { ffi::waddch(w, ch) }
}

pub fn waddstr(w: WINDOW, s: &str) -> i32 {
    let c_string = CString::new(s).unwrap();
    unsafe { ffi::waddstr(w, c_string.as_ptr()) }
}

pub fn wclear(w: WINDOW) -> i32 {
    unsafe { ffi::wclear(w) }
}

pub fn wrefresh(w: WINDOW) -> i32 {
    unsafe { ffi::wrefresh(w) }
}
