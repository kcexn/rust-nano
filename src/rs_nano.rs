extern crate ncurses;

use ncurses::*;

const CTRL_D: i32 = 4;

pub fn init() {
    initscr();
    raw();
    keypad(stdscr(),true);
    noecho();
    return;
}

pub fn run() {
    /* TODO: Fix a bug where using a new line breaks 
     * the backspace button */
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    loop {
        let c = getch();
        if c == CTRL_D {
            break;
        } else if c == KEY_BACKSPACE {
            if x == 0 {
                mvdelch(y-1,COLS()-1);
            } else {
                mvdelch(y,x-1);
            }
            getyx(stdscr(),&mut y, &mut x);
        } else {
            addch(c as chtype);
            getyx(stdscr(),&mut y,&mut x);
        }
        refresh();
    }
    return;
}

pub fn cleanup() {
    endwin();
    return;
}
