extern crate ncurses;

use ncurses::*;

const CTRL_D: i32 = 4;

fn main() {
    /* track the cursor position using these variables */
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    /* initialise the default screen stdscr */
    initscr();
    raw();
    keypad(stdscr(),true);
    noecho();
    /* main window loop */
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
    /* clean up before returning an exit code */
    endwin();
    std::process::exit(0);
}
