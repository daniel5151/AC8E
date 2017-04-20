use disasm;

pub trait Chip8Utils {
    fn disasm(&self) -> String;
    fn nibble_at(&self, i: u8) -> u8;
}

impl Chip8Utils for u16 {
    fn disasm(&self) -> String {
        disasm::disasm(*self)
    }
    fn nibble_at(&self, i: u8) -> u8 {
        (match i {
             3 => (*self & 0x000F),
             2 => (*self & 0x00F0) >> 4,
             1 => (*self & 0x0F00) >> 8,
             0 => (*self & 0xF000) >> 12,
             // if this happens, it's my own fault
             _ => panic!("Cannot get {}th nibble from Word (u16)!", i),
         }) as u8
    }
}
