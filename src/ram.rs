#![allow(dead_code, unused_variables)]

pub struct RAM {
    mem: [u8; 0x1000],
}

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; 0x1000] }
    }

    pub fn load_u16(&self, addr: u16) -> Result<u16, String> {
        match addr {
            _ if addr >= 0xFFF => {
                return Err(format!("[RAM] Address 0x{:03x} is out of bounds!",
                                   addr));
            }
            _ if addr % 2 != 0 => {
                return Err(format!("[RAM] Address 0x{:03x} is misaligned!",
                                   addr));
            }
            _ => (), // all clear
        }

        let i = addr as usize;
        let word = (self.mem[i] as u16) << 8 | (self.mem[i + 1] as u16);

        Ok(word)
    }

    pub fn store_u16(&mut self, addr: u16, val: u16) -> Result<(), String> {
        match addr {
            _ if addr >= 0xFFF => {
                return Err(format!("[RAM] Address 0x{:03x} is out of bounds!",
                                   addr));
            }
            _ if addr % 2 != 0 => {
                return Err(format!("[RAM] Address 0x{:03x} is misaligned!",
                                   addr));
            }
            _ => (), // all clear
        }

        let i = addr as usize;
        self.mem[i + 0] = (val >> 8) as u8;
        self.mem[i + 1] = (val >> 0) as u8;

        Ok(())
    }

    pub fn load_u8(&self, addr: u16) -> Result<u8, String> {
        if addr >= 0xFFF {
            return Err(format!("[RAM] Address 0x{:03x} is out of bounds!",
                               addr));
        }

        Ok(self.mem[addr as usize])
    }

    pub fn store_u8(&mut self, addr: u16, val: u8) -> Result<(), String> {
        if addr >= 0xFFF {
            return Err(format!("[RAM] Address 0x{:03x} is out of bounds!",
                               addr));
        }

        self.mem[addr as usize] = val;
        Ok(())
    }
}

// not used *yet*
#[cfg_attr(rustfmt, rustfmt_skip)]
static FONTSET: [u8; 80] = [
  /* 0 */ 0xF0, 0x90, 0x90, 0x90, 0xF0,
  /* 1 */ 0x20, 0x60, 0x20, 0x20, 0x70,
  /* 2 */ 0xF0, 0x10, 0xF0, 0x80, 0xF0,
  /* 3 */ 0xF0, 0x10, 0xF0, 0x10, 0xF0,
  /* 4 */ 0x90, 0x90, 0xF0, 0x10, 0x10,
  /* 5 */ 0xF0, 0x80, 0xF0, 0x10, 0xF0,
  /* 6 */ 0xF0, 0x80, 0xF0, 0x90, 0xF0,
  /* 7 */ 0xF0, 0x10, 0x20, 0x40, 0x40,
  /* 8 */ 0xF0, 0x90, 0xF0, 0x90, 0xF0,
  /* 9 */ 0xF0, 0x90, 0xF0, 0x10, 0xF0,
  /* A */ 0xF0, 0x90, 0xF0, 0x90, 0x90,
  /* B */ 0xE0, 0x90, 0xE0, 0x90, 0xE0,
  /* C */ 0xF0, 0x80, 0x80, 0x80, 0xF0,
  /* D */ 0xE0, 0x90, 0x90, 0x90, 0xE0,
  /* E */ 0xF0, 0x80, 0xF0, 0x80, 0xF0,
  /* F */ 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
