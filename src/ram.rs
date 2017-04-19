#![allow(dead_code, unused_variables)]

use types::*;

#[derive(Debug)]
pub enum RAMError {
    OutOfBoundsAccess,
    MisalignedAccess,
}

pub struct RAM {
    mem: [u8; 0x1000],
}

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; 0x1000] }
    }

    pub fn load(&self, addr: Word) -> Result<Word, RAMError> {
        match addr {
            _ if addr >= 0xFFF => return Err(RAMError::OutOfBoundsAccess),
            _ if addr % 2 != 0 => return Err(RAMError::MisalignedAccess),
            _ => (), // all clear
        }

        let i = addr as usize;
        let word = (self.mem[i] as Word) << 8 | (self.mem[i + 1] as Word);

        Ok(word)
    }

    pub fn store(&mut self, addr: Word, val: Word) -> Result<(), RAMError> {
        match addr {
            _ if addr >= 0xFFF => return Err(RAMError::OutOfBoundsAccess),
            _ if addr % 2 != 0 => return Err(RAMError::MisalignedAccess),
            _ => (), // all clear
        }

        let i = addr as usize;
        self.mem[i + 0] = (val >> 8) as u8;
        self.mem[i + 1] = (val >> 0) as u8;

        Ok(())
    }
}

// not used *yet*
static FONTSET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
