extern crate ncurses;

use ncurses::*;
use std::char;
use std::string;
use std::convert::TryFrom;
use std::io::prelude::*;

fn main() {
    let mut stdin = std::io::stdin();

    initscr();
    keypad(stdscr(),true);
    noecho();
    for i in 0..10 {
        let c = getch();
        // addch implicitly calls refresh which is not confusing at all.
        addch(c as u64);
    }
    let _ = stdin.read(&mut [0u8]).unwrap(); 
    // Here we can see that an explicit call to refresh is required.
    refresh();
    let _ = stdin.read(&mut [0u8]).unwrap();
    endwin();
    std::process::exit(0);
}
