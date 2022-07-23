use std::{thread, time};

use ncurses_lite::*;

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let w = newwin(20, 100, 1, 1);

    waddstr(w, "Hello");
    waddch(w, '\n' as u32);
    waddstr(w, "World!");

    thread::sleep(time::Duration::from_millis(3000));

    wclear(w);

    waddstr(w, "Okay, bye now!");

    thread::sleep(time::Duration::from_millis(3000));

    endwin();
}
