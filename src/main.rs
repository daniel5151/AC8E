use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;

mod types;
mod disasm;
mod ram;

use types::*;

fn main() {
    let path = Path::new("roms/games/GUESS");
    let display = path.display();

    let f = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    };

    // convert the file from a bytestream to a vector of u16 (chip8 words)
    let rom: Vec<Word> = f.bytes()
        .map(|x| x.unwrap())
        .collect::<Vec<u8>>() // have to manually specify collection type
        .chunks(2)
        .map(|w| ((w[0] as u16) << 8) | (w[1] as u16))
        .collect();

    for (i, word) in rom.iter().enumerate() {
        print!("0x{:03x} : 0x{:04x} : ", i * 2, word);
        print!("{}\n", word.clone().disasm());
    }
}
