#![allow(dead_code, unused_variables)]

pub trait Update {
    fn clear(&self);
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool;
}

pub trait Render {
    fn render(&self);
}

use std::cell::RefCell;
pub struct TermDisplay {
    pixels: RefCell<[[bool; 64]; 32]>,
}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay { pixels: RefCell::new([[false; 64]; 32]) }
    }
}

// TODO: nice (SDL prolly) display
// pub struct GfxDisplay {
//     pixels: RefCell<[[bool; 64]; 32]>,
// }
// impl GfxDisplay {
//     pub fn new() -> GfxDisplay {
//         GfxDisplay { pixels: RefCell::new([[false; 64]; 32]) }
//     }
// }

// Chip-8 draws graphics on screen through the use of sprites.
// A sprite is a group of bytes which are a binary representation of the desired
// picture.
// Chip-8 sprites may be up to 15 bytes, for a possible sprite size of 8x15.
impl Update for TermDisplay {
    fn clear(&self) {
        *self.pixels.borrow_mut() = [[false; 64]; 32];
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        for (row, byte) in ram.iter().enumerate() {
            // Transform byte into bitvector
            let bits = (0..8)
                .rev()
                .map(|x| ((*byte >> x) % 2) == 1)
                .collect::<Vec<bool>>();

            for (i, xi) in (x..(x + 8)).enumerate() {
                let mut p = self.pixels.borrow_mut();
                p[(y as usize + row) % 32][(xi % 64) as usize] ^= bits[i];
            }
        }
        // nothing yet
        false
    }
}

impl Render for TermDisplay {
    fn render(&self) {
        print!("\x1b[2J\x1b[1;1H"); // clear screen magic
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
