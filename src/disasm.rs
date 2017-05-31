use types::*;

pub fn disasm(word: u16) -> String {
    // these aren't used in *every* instruction, but they are nice values to
    // calculate. It helps keep the code clean :)
    let x = word.nibble_at(1);
    let y = word.nibble_at(2);
    let nnn = word & 0x0FFF;
    let kk = word & 0x00FF;

    match word.nibble_at(0) {
        0x0 if nnn == 0x0E0 => format!("CLS"),
        0x0 if nnn == 0x0EE => format!("RET"),
        0x0 => format!(".word   0x{:04x}", word),
        0x1 => format!("JP      0x{:03x}", nnn),
        0x2 => format!("CALL    0x{:03x}", nnn),
        0x3 => format!("SE      V{:x}, {}", x, kk),
        0x4 => format!("SNE     V{:x}, {}", x, kk),
        0x5 => format!("SE      V{:x}, V{:x}", x, y),
        0x6 => format!("LD      V{:x}, {}", x, kk),
        0x7 => format!("ADD     V{:x}, {}", x, kk),
        0x8 => match word.nibble_at(3) {
            0x0 => format!("LD      V{:x} V{:x}", x, y),
            0x1 => format!("OR      V{:x} V{:x}", x, y),
            0x2 => format!("AND     V{:x} V{:x}", x, y),
            0x3 => format!("XOR     V{:x} V{:x}", x, y),
            0x4 => format!("ADD     V{:x} V{:x}", x, y),
            0x5 => format!("SUB     V{:x} V{:x}", x, y),
            0x6 => format!("SHR     V{:x} V{:x}", x, y),
            0x7 => format!("SUBN    V{:x} V{:x}", x, y),
            0xE => format!("SHL     V{:x} V{:x}", x, y),
            _ => format!(".word   0x{:04x}", word),
        },
        0x9 => format!("SNE     V{:x} V{:x}", x, y),
        0xA => format!("LD      I, 0x{:03x}", nnn),
        0xB => format!("JP      V0, 0x{:03x}", nnn),
        0xC => format!("RND     V{:x}, {}", x, kk),
        0xD => format!("DRW     V{:x}, V{:x}, {}", x, y, word.nibble_at(3)),
        0xE if kk == 0x9E => format!("SKP     V{:x} << INPUT", x),
        0xE if kk == 0xA1 => format!("SKNP    V{:x} << INPUT", x),
        0xE => format!(".word   0x{:04x}", word),
        0xF => match kk {
            0x07 => format!("LD      V{:x}, DT", x),
            0x0A => format!("LD      V{:x}, K << INPUT", x),
            0x15 => format!("LD      DT, V{:x}", x),
            0x18 => format!("LD      ST, V{:x}", x),
            0x1E => format!("ADD     I, V{:x}", x),
            0x29 => format!("LD      F, V{:x}", x),
            0x33 => format!("LD      B, V{:x}", x),
            0x55 => format!("LD      [I], V{:x}", x),
            0x65 => format!("LD      V{:x}, [I]", x),
            _ => format!(".word   0x{:04x}", word),
        },
        _ => format!(""),
    }
}
