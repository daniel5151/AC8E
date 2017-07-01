#![allow(dead_code)]

extern crate ncurses;

use std::cell::RefCell;

/*======================================
=            Display Traits            =
======================================*/

// Trait exposed to CPU to allow updating underlying screen memory
pub trait Update {
    // clear screen
    fn clear(&self);
    // Draw to screen RAM according to chip8 spec
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool;
}

// Trait exposed to main loop to actually render the screen
pub trait Render {
    fn init(&self); // run once at start
    fn uninit(&self); // run once at end
    fn render(&self, beep: bool); // run every frame
}

/*==================================
=            Screen RAM            =
==================================*/
// This struct implements the underlying DRAW instruction logic, and holds the
// bit-arrays that represent the screen.

struct ScreenRAM {
    pixels: RefCell<[[bool; 64]; 32]>,
}

impl ScreenRAM {
    pub fn new() -> ScreenRAM {
        ScreenRAM { pixels: RefCell::new([[false; 64]; 32]) }
    }
}

impl Update for ScreenRAM {
    fn clear(&self) {
        *self.pixels.borrow_mut() = [[false; 64]; 32];
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        let mut collision = false;

        for (row, byte) in ram.iter().enumerate() {
            // Transform byte into bitvector
            let bits = (0..8)
                .rev()
                .map(|x| ((*byte >> x) % 2) == 1)
                .collect::<Vec<bool>>();

            for bit in 0..8usize {
                let y = (y as usize + row) % 32;
                let x = (x as usize + bit) % 64;

                let mut p = self.pixels.borrow_mut();

                // check collision
                if p[y][x] == true && bits[bit] == true {
                    collision = true;
                }

                // do the xor
                p[y][x] ^= bits[bit];
            }
        }

        collision
    }
}

/*=================================
=            Renderers            =
=================================*/

// Renderer structs are just simple wrappers around ScreenRAM.
// The `Update` trait simply calls the equivalent Screen RAM's functions, and
// the `Render` trait is used to implement different rendering modes

/* ----------  Null Renderer  ---------- */

pub struct NullDisplay {
    screen: ScreenRAM,
}

impl NullDisplay {
    pub fn new() -> NullDisplay {
        NullDisplay { screen: ScreenRAM::new() }
    }
}

// simply call the underlying screenRAM methods
impl Update for NullDisplay {
    fn clear(&self) {
        self.screen.clear()
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        self.screen.draw(x, y, ram)
    }
}

impl Render for NullDisplay {
    fn init(&self) {}
    fn uninit(&self) {}
    fn render(&self, beep: bool) {
        drop(beep)
    }
}

/* ----------  Terminal Renderer  ---------- */
// Basic renderer to output to terminal.
// ** Cannot be extended with associated realtime input!

pub struct TermDisplay {
    screen: ScreenRAM,
}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay { screen: ScreenRAM::new() }
    }
}

// simply call the underlying screenRAM methods
impl Update for TermDisplay {
    fn clear(&self) {
        self.screen.clear()
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        self.screen.draw(x, y, ram)
    }
}

impl Render for TermDisplay {
    fn init(&self) {}
    fn uninit(&self) {}

    fn render(&self, beep: bool) {
        print!("\x1b[2J\x1b[1;1H"); // magic chars to clear the term screen

        for y in 0..32 {
            for x in 0..64 {
                print!("{}",
                       format!("{}", self.screen.pixels.borrow()[y][x] as u8)
                           .replace("0", " ")
                           .replace("1", "X"));
            }
            println!();
        }

        if beep {
            println!("\u{0007}");
        }
    }
}


/* ----------  Ncurses Renderer  ---------- */
// ncurses based terminal renderer
// faster, and supports realtime input

pub struct NcursesDisplay {
    screen: ScreenRAM,
}

impl NcursesDisplay {
    pub fn new() -> NcursesDisplay {
        NcursesDisplay { screen: ScreenRAM::new() }
    }
}

// simply call the underlying screenRAM methods
impl Update for NcursesDisplay {
    fn clear(&self) {
        self.screen.clear()
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        self.screen.draw(x, y, ram)
    }
}

use self::ncurses as nc;

impl Render for NcursesDisplay {
    fn init(&self) {
        /* Setup ncurses. */
        nc::initscr();
        nc::raw();

        /* Allow for extended keyboard (like F1). */
        nc::keypad(nc::stdscr(), true);
        nc::noecho();

        /* Invisible cursor. */
        nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }
    fn uninit(&self) {
        /* Kill ncurses. */
        nc::endwin();
    }

    fn render(&self, beep: bool) {

        nc::mv(0, 0);

        for y in 0..32 {
            for x in 0..64 {
                nc::printw(format!("{}",
                                   self.screen.pixels.borrow()[y][x] as u8)
                                   .replace("0", " ")
                                   .replace("1", "X")
                                   .as_ref());
            }
            nc::printw("\n");
        }

        if beep {
            nc::beep();
        }

        nc::refresh();
    }
}
