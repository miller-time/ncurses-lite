#![allow(non_camel_case_types)]

use libc::c_int;

pub type chtype = u32;

pub enum CURSOR_VISIBILITY {
    CURSOR_INVISIBLE,
}

pub type WINDOW = *mut i8;

pub fn initscr() {}

pub fn endwin() {}

pub fn curs_set(_visibility: CURSOR_VISIBILITY) {}

pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW {
    unsafe { c_newwin(lines, cols, y, x) }
}

pub fn waddch(_w: WINDOW, _ch: chtype) {}

pub fn waddstr(_w: WINDOW, _s: &str) {}

pub fn wclear(_w: WINDOW) {}

pub fn wrefresh(_w: WINDOW) {}

extern "C" {
    fn c_newwin(_: c_int, _: c_int, _: c_int, _: c_int) -> WINDOW;
}
