#![allow(dead_code, unused_variables)]

use std::cell::RefCell;

/* ----------  Display Traits  ---------- */

// The CPU only sees the display's clear and draw functions, and cannot invoke
// any rendering
pub trait Update {
    fn clear(&self);
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool;
}

pub trait Render {
    fn render(&self);
}

/* ----------  Terminal Renderer  ---------- */

pub struct TermDisplay {
    pixels: RefCell<[[bool; 64]; 32]>,
}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay { pixels: RefCell::new([[false; 64]; 32]) }
    }
}

impl Update for TermDisplay {
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


            for (i, xi) in (x..(x + 8)).enumerate() {
                let y = (y as usize + row) % 32;
                let x = (xi % 64) as usize;

                let mut p = self.pixels.borrow_mut();

                // check collision
                if p[y][x] == true && bits[i] == true {
                    collision = true;
                }

                // do the xor
                p[y][x] ^= bits[i];
            }
        }

        collision
    }
}

impl Render for TermDisplay {
    fn render(&self) {
        print!("\x1b[2J\x1b[1;1H"); // magic chars to clear the term screen

        for y in 0..32 {
            for x in 0..64 {
                print!("{}",
                       format!("{}", self.pixels.borrow()[y][x] as u8)
                           .replace("0", " ")
                           .replace("1", "█"));
            }
            println!();
        }
    }
}
