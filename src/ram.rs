pub struct RAM {
    mem: [u8; 0x1000],
}

use types::Chip8Utils;

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; 0x1000] }
    }

    pub fn print_around(&self, addr: u16) {
        let addrs = (-3..3)
            .map(|x| addr as i32 + x * 2)
            .filter(|x| *x >= 0 && *x <= 0xFFF)
            .map(|x| x as u16);

        for _addr in addrs {
            let val = self.load_u16(_addr).unwrap();
            println!("{} 0x{:03x} | 0x{:04x} | {}",
                     if _addr == addr { ">" } else { " " },
                     _addr,
                     val,
                     val.disasm());
        }
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
