#![allow(dead_code, unused_variables)]

// Chip-8 draws graphics on screen through the use of sprites.
// A sprite is a group of bytes which are a binary representation of the desired
// picture.
// Chip-8 sprites may be up to 15 bytes, for a possible sprite size of 8x15.

struct Screen([u8; 64 * 32]);

pub struct TermDisplay(Screen);
impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay(Screen([0; 64 * 32]))
    }
}

pub struct GfxDisplay(Screen);
impl GfxDisplay {
    pub fn new() -> GfxDisplay {
        GfxDisplay(Screen([0; 64 * 32]))
    }
}

pub trait Display {
    fn clear(&self);
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool;
}

impl Display for TermDisplay {
    fn clear(&self) {
        println!("  Called TermDisplay `clear`");
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        println!("  Called TermDisplay `draw`");
        false
    }
}

impl Display for GfxDisplay {
    fn clear(&self) {
        println!("  Called GfxDisplay `clear`");
    }
    fn draw(&self, x: u8, y: u8, ram: &[u8]) -> bool {
        println!("  Called GfxDisplay `draw`");
        false
    }
}
