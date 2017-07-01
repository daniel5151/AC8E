#![allow(dead_code)]

pub struct RAM {
    mem: [u8; 0x1000],
}

fn err_oob(addr: u16) -> String {
    format!("[RAM] Address 0x{:03x} is out of bounds!", addr)
}

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; 0x1000] }
    }

    pub fn load_u16(&self, addr: u16) -> Result<u16, String> {
        if addr >= 0xFFF {
            return Err(err_oob(addr));
        }

        let i = addr as usize;
        let word = (self.mem[i] as u16) << 8 | (self.mem[i + 1] as u16);

        Ok(word)
    }

    pub fn store_u16(&mut self, addr: u16, val: u16) -> Result<(), String> {
        if addr >= 0xFFF {
            return Err(err_oob(addr));
        }

        let i = addr as usize;
        self.mem[i + 0] = (val >> 8) as u8;
        self.mem[i + 1] = (val >> 0) as u8;

        Ok(())
    }

    pub fn load_u8(&self, addr: u16) -> Result<u8, String> {
        if addr >= 0xFFF {
            return Err(err_oob(addr));
        }

        Ok(self.mem[addr as usize])
    }

    pub fn store_u8(&mut self, addr: u16, val: u8) -> Result<(), String> {
        if addr >= 0xFFF {
            return Err(err_oob(addr));
        }

        self.mem[addr as usize] = val;
        Ok(())
    }
}
