#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use libc::{c_char, c_int};

pub type WINDOW = *mut i8;

#[link(name = "ncurses")]
extern "C" {
    pub(crate) fn initscr() -> WINDOW;
    pub(crate) fn endwin() -> c_int;
    pub(crate) fn curs_set(_: c_int) -> c_int;
    pub(crate) fn newwin(_: c_int, _: c_int, _: c_int, _: c_int) -> WINDOW;
    pub(crate) fn waddch(_: WINDOW, _: u32) -> c_int;
    pub(crate) fn waddstr(_: WINDOW, _: *const c_char) -> c_int;
    pub(crate) fn wclear(_: WINDOW) -> c_int;
    pub(crate) fn wrefresh(_: WINDOW) -> c_int;
}
