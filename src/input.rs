#![allow(dead_code, unused_variables)]

extern crate ncurses;

use std::cell::Cell;
use std::cell::RefCell;
use std::char;

/*====================================
=            Input Traits            =
====================================*/

pub trait Get {
    fn pressed_esc(&self) -> bool; // Checks if exit key was pressed

    fn pressed_key(&self, key: u8) -> bool; // Checks if key 0-F was pressed
    fn last_press(&self) -> Option<u8>;
}

pub trait Set {
    fn decrement_keys(&self);
    fn update_keys(&self, block: bool); // updates key list with pressed keys
}

/*=====================================
=            Input Methods            =
=====================================*/

/* ----------  Null Input  ---------- */

pub struct NullInput {}

impl NullInput {
    pub fn new() -> NullInput {
        NullInput {}
    }
}

impl Get for NullInput {
    fn pressed_esc(&self) -> bool {
        false
    }

    fn pressed_key(&self, key: u8) -> bool {
        false
    }
    fn last_press(&self) -> Option<u8> {
        None
    }
}

impl Set for NullInput {
    fn decrement_keys(&self) {}
    fn update_keys(&self, block: bool) {}
}

/* ----------  Ncurses Input  ---------- */

pub struct NcursesInput {
    keys: RefCell<[u8; 16]>,
    exit: Cell<bool>,

    last_press: Cell<Option<u8>>,
}

impl NcursesInput {
    pub fn new() -> NcursesInput {
        NcursesInput {
            keys: RefCell::new([0; 16]),
            exit: Cell::new(false),

            last_press: Cell::new(None),
        }
    }
}

impl Get for NcursesInput {
    fn pressed_esc(&self) -> bool {
        self.exit.get()
    }

    fn pressed_key(&self, key: u8) -> bool {
        if key > 0xF {
            return false;
        }

        self.keys.borrow()[key as usize] != 0
    }

    fn last_press(&self) -> Option<u8> {
        // Give back the key just pressed, and set self to None
        let last_press = self.last_press.get();
        self.last_press.set(None);
        last_press
    }
}

use self::ncurses as nc;

impl Set for NcursesInput {
    fn decrement_keys(&self) {
        for x in self.keys.borrow_mut().iter_mut() {
            *x -= if *x > 0 { 1 } else { 0 }
        }
    }

    fn update_keys(&self, block: bool) {

        nc::timeout(if block { -1 } else { 0 });
        let input = nc::getch();

        // Match special values
        match input {
            // Exit Key
            nc::KEY_F1 => {
                self.exit.set(true);
                return;
            }
            // No input
            -1 => return,
            // Otherwise, push forwards
            _ => (),
        }

        // Keymap

        let key_pressed = match char::from_u32(input as u32) {
            #[cfg_attr(rustfmt, rustfmt_skip)] // keep the 4x4
            Some(c) => match c {
                '1' => 0x1, '2' => 0x2, '3' => 0x3, '4' => 0xC,
                'q' => 0x4, 'w' => 0x5, 'e' => 0x6, 'r' => 0xD,
                'a' => 0x7, 's' => 0x8, 'd' => 0x9, 'f' => 0xE,
                'z' => 0xA, 'x' => 0x0, 'c' => 0xB, 'v' => 0xF,
                _ => return,
            },
            None => return,
        };

        // Recall that this key was just pressed
        self.last_press.set(Some(key_pressed));

        // This number controls "key stickieness"
        self.keys.borrow_mut()[key_pressed as usize] = 8;
    }
}
