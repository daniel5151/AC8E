extern crate rand;

use display;
use input;
use ram;
use types::Chip8Utils;

#[derive(PartialEq)]
pub enum CPUState {
    Running,
    WaitForInput,
}

pub struct CPU<'a, Dt: 'a + display::Update, It: 'a + input::Get> {
    cycle: u32,

    v: [u8; 16],
    i: u16,

    pc: u16,
    stack: Vec<u16>,

    dt: u8,
    st: u8,

    ram: &'a mut ram::RAM,
    display: &'a Dt,
    input: &'a It,
}

impl<'a, Dt: display::Update, It: input::Get> CPU<'a, Dt, It> {
    pub fn new(ram: &'a mut ram::RAM,
               display: &'a Dt,
               input: &'a It)
               -> CPU<'a, Dt, It> {
        // Load FONTSET into RAM
        for (i, byte) in FONTSET.iter().enumerate() {
            // i'm just going to unwrap this value, since I know it won't fail.
            ram.store_u8(i as u16, *byte).unwrap();
        }

        CPU {
            cycle: 0,

            v: [0; 16],
            i: 0,

            pc: 0x200,
            stack: vec![],

            dt: 0,
            st: 0,

            ram: ram,
            display: display,
            input: input,
        }
    }

    pub fn decrement_counters(&mut self) {
        self.dt -= if self.dt > 0 { 1 } else { 0 };
        self.st -= if self.st > 0 { 1 } else { 0 };
    }

    pub fn cycle(&mut self) -> Result<CPUState, String> {
        self.cycle += 1;

        // Load instr from RAM
        let instr = self.ram.load_u16(self.pc)?;
        self.pc += 2;

        // these values aren't used in *every* instruction, but they are nice
        // to have on hand. It helps keep the code clean :)
        let x = instr.nibble_at(1) as usize;
        let y = instr.nibble_at(2) as usize;
        let nnn = instr & 0x0FFF;
        let kk = (instr & 0x00FF) as u8;

        match instr.nibble_at(0) {
            // 00E0 - CLS
            // Clear the display.
            0x0 if nnn == 0x0E0 => self.display.clear(),
            // 00EE - RET
            // Return from a subroutine.
            // The interpreter sets the program counter to the address at
            // the top of the stack, then subtracts 1 from the stack pointer
            0x0 if nnn == 0x0EE => {
                // I'm assuming you can't RET when the stack is clear...
                self.pc = match self.stack.pop() {
                    Some(addr) => addr,
                    None => return Err("[CPU] Cannot RET when stack is empty!"
                                           .to_string()),
                };
            }
            // 0nnn - SYS addr
            // Jump to a machine code routine at nnn.
            // This instruction is only used on the old computers on which
            // Chip-8 was originally implemented. It is ignored by modern
            // interpreters.
            0x0 => (),
            // 1nnn - JP addr
            // Jump to location nnn.
            // The interpreter sets the program counter to nnn.
            0x1 => self.pc = nnn,
            // 2nnn - CALL addr
            // Call subroutine at nnn.
            // The interpreter increments the stack pointer, then puts the
            // current PC on the top of the stack. The PC is then set to nnn
            0x2 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            // The interpreter compares register Vx to kk, and if they are
            // equal, increments the program counter by 2.
            0x3 => self.pc += if self.v[x] == kk { 2 } else { 0 },
            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            // The interpreter compares register Vx to kk, and if they are
            // not equal, increments the program counter by 2.
            0x4 => self.pc += if self.v[x] != kk { 2 } else { 0 },
            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            // The interpreter compares register Vx to register Vy, and if
            // they are equal, increments the program counter by 2.
            0x5 => self.pc += if self.v[x] == self.v[y] { 2 } else { 0 },
            // 6xkk - LD Vx, byte
            // Set Vx = kk.
            // The interpreter puts the value kk into register Vx.
            0x6 => self.v[x] = kk,
            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            // Adds the value kk to the value of register Vx, then stores the
            // result in Vx.
            0x7 => self.v[x] = self.v[x].wrapping_add(kk),
            0x8 => match instr.nibble_at(3) {
                // 8xy0 - LD Vx, Vy
                // Set Vx = Vy.
                // Stores the value of register Vy in register Vx.
                0x0 => self.v[x] = self.v[y],
                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                // Performs a bitwise OR on the values of Vx and Vy,
                // then stores the result in Vx.
                0x1 => self.v[x] = self.v[x] | self.v[y],
                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.
                // Performs a bitwise AND on the values of Vx and Vy,
                // then stores the result in Vx.
                0x2 => self.v[x] = self.v[x] & self.v[y],
                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.
                // Performs a bitwise exclusive OR on the values of Vx
                // and Vy, then stores the result in Vx.
                0x3 => self.v[x] = self.v[x] ^ self.v[y],
                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.
                // The values of Vx and Vy are added together.
                // If the result is greater than 8 bits (i.e., > 255,)
                // VF is set to 1, otherwise 0. Only the lowest 8 bits
                // of the result are kept, and stored in Vx.
                0x4 => {
                    let add = self.v[x] as u16 + self.v[y] as u16;
                    self.v[0xF] = (add > 0xFF) as u8;
                    self.v[x] = add as u8;
                }
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy
                // is subtracted from Vx, and the results stored in Vx.
                0x5 => {
                    self.v[0xF] = (self.v[x] > self.v[y]) as u8;
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                }
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                // If the least-significant bit of Vx is 1, then VF is
                // set to 1, otherwise 0. Then Vx is divided by 2.
                0x6 => {
                    self.v[0xF] = self.v[x] & 0x01;
                    self.v[x] >>= 1;
                }
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx
                // is subtracted from Vy, and the results stored in Vx.
                0x7 => {
                    self.v[0xF] = (self.v[y] > self.v[x]) as u8;
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                }
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                // If the most-significant bit of Vx is 1, then VF is
                // set to 1, otherwise to 0. Then Vx is multiplied by 2.
                0xE => {
                    self.v[0xF] = self.v[x] >> 7;
                    self.v[x] <<= 1;
                }
                _ => return Err("[CPU] Invalid Opcode".to_string()),
            },
            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            // The values of Vx and Vy are compared, and if they are not
            // equal, the program counter is increased by 2.
            0x9 => self.pc += if self.v[x] != self.v[y] { 2 } else { 0 },
            // Annn - LD I, addr
            // Set I = nnn.
            // The value of register I is set to nnn.
            0xA => self.i = nnn,
            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            // The program counter is set to nnn plus the value of V0.
            0xB => self.pc = nnn + self.v[0] as u16,
            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255,
            // which is then ANDed with the value kk. The results are stored
            // in Vx. See instruction 8xy2 for more information on AND.
            0xC => self.v[x] = rand::random::<u8>() & kk,
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at
            // (Vx, Vy), set VF = collision.
            // The interpreter reads n bytes from memory, starting at the
            // address stored in I.
            // These bytes are then displayed as sprites on screen at
            // coordinates (Vx, Vy).
            // Sprites are XORed onto the existing screen.
            // If this causes any pixels to be erased, VF is set to 1, otherwise
            // it is set to 0.
            // If the sprite is positioned so part of it is outside the
            // coordinates of the display, it wraps around to the opposite side
            // of the screen.
            0xD => {
                // check for unexpected overflows of the I register
                if (self.i + instr.nibble_at(3) as u16) > 0xFFF {
                    return Err("[CPU] Unexpected Overflow".to_string());
                }

                let sprite = (self.i..(self.i + instr.nibble_at(3) as u16))
                    .map(|addr| self.ram.load_u8(addr))
                    .collect::<Result<Vec<u8>, _>>()?;

                self.v[0xF] =
                    self.display.draw(self.v[x], self.v[y], &sprite) as u8;
            }
            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            // Checks the keyboard, and if the key corresponding to the
            // value of Vx is currently in the down position, PC is
            // increased by 2.
            0xE if kk == 0x9E => {
                if self.input.pressed_key(self.v[x]) {
                    self.pc += 2;
                }
            }
            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            // Checks the keyboard, and if the key corresponding to the
            // value of Vx is currently in the up position, PC is increased
            // by 2.
            0xE if kk == 0xA1 => {
                if !self.input.pressed_key(self.v[x]) {
                    self.pc += 2;
                }
            }
            0xE => return Err("[CPU] Invalid Opcode".to_string()),
            0xF => match kk {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                // The value of DT is placed into Vx.
                0x07 => self.v[x] = self.dt,
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key
                // in Vx.
                // All execution stops until a key is pressed, then the
                // value of that key is stored in Vx.
                0x0A => {
                    // I hate this instruction.
                    // It makes my life so incredibly difficult...
                    match self.input.last_press() {
                        Some(key) => self.v[x] = key,
                        None => {
                            self.cycle -= 1;
                            self.pc -= 2;
                            return Ok(CPUState::WaitForInput);
                        }
                    }
                }
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx.
                // DT is set equal to the value of Vx.
                0x15 => self.dt = self.v[x],
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                // ST is set equal to the value of Vx.
                0x18 => self.st = self.v[x],
                // Fx1E - ADD I, Vx
                // Set I = I + Vx.
                // The values of I and Vx are added, and the results are
                // stored in I.
                0x1E => self.i = self.i + self.v[x] as u16,
                // Fx29 - LD F, Vx
                // Set I = location of sprite for digit Vx.
                // The value of I is set to the location for the
                // hexadecimal sprite corresponding to the value of Vx.
                0x29 if self.v[x] <= 0xF => self.i = self.v[x] as u16 * 5,
                0x29 => return Err("[CPU] Invalid Opcode".to_string()),
                // Fx33 - LD B, Vx
                // Store BCD representation of Vx in memory locations I,
                // I+1, and I+2.
                // The interpreter takes the decimal value of Vx, and
                // places the hundreds digit in memory at location in I,
                // the tens digit at location I+1, and the ones digit at
                // location I+2.
                0x33 => {
                    self.ram.store_u8(self.i + 0, self.v[x] / 100 % 10)?;
                    self.ram.store_u8(self.i + 1, self.v[x] / 10 % 10)?;
                    self.ram.store_u8(self.i + 2, self.v[x] / 1 % 10)?;
                }
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at
                // location I.
                // The interpreter copies the values of registers V0
                // through Vx into memory, starting at the address in I.
                0x55 => {
                    for x in 0..(x + 1) {
                        self.ram.store_u8(self.i + x as u16, self.v[x])?;
                    }
                }
                // Fx65 - LD Vx, [I]
                // Read registers V0 through Vx from memory starting at
                // location I.
                // The interpreter reads values from memory starting at
                // location I into registers V0 through Vx.
                0x65 => {
                    for x in 0..(x + 1) {
                        self.v[x] = self.ram.load_u8(self.i + x as u16)?;
                    }
                }
                _ => return Err("[CPU] Invalid Opcode".to_string()),
            },
            // this will never happen, but Rust doesn't know what a nibble is,
            // so it thinks that there are more cases that need to be checked.
            // Ah well
            _ => (),
        };

        Ok(CPUState::Running)
    }
}

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
