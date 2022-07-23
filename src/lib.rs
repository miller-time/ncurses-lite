#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::CString;

use libc::{c_char, c_int};

pub enum CURSOR_VISIBILITY {
    CURSOR_INVISIBLE,
}

pub type WINDOW = *mut i8;

pub fn initscr() -> WINDOW {
    unsafe { c_initscr() }
}

pub fn endwin() -> i32 {
    unsafe { c_endwin() }
}

pub fn curs_set(visibility: CURSOR_VISIBILITY) -> i32 {
    unsafe { c_curs_set(visibility as i32) }
}

pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW {
    unsafe { c_newwin(lines, cols, y, x) }
}

pub fn waddch(w: WINDOW, ch: u32) -> i32 {
    unsafe { c_waddch(w, ch) }
}

pub fn waddstr(w: WINDOW, s: &str) -> i32 {
    let c_string = CString::new(s).unwrap();
    unsafe { c_waddstr(w, c_string.as_ptr()) }
}

pub fn wclear(w: WINDOW) -> i32 {
    unsafe { c_wclear(w) }
}

pub fn wrefresh(w: WINDOW) -> i32 {
    unsafe { c_wrefresh(w) }
}

extern "C" {
    fn c_initscr() -> WINDOW;
    fn c_endwin() -> c_int;
    fn c_curs_set(_: c_int) -> c_int;
    fn c_newwin(_: c_int, _: c_int, _: c_int, _: c_int) -> WINDOW;
    fn c_waddch(_: WINDOW, _: u32) -> c_int;
    fn c_waddstr(_: WINDOW, _: *const c_char) -> c_int;
    fn c_wclear(_: WINDOW) -> c_int;
    fn c_wrefresh(_: WINDOW) -> c_int;
}
