use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod cpu;
mod disasm;
mod display;
mod ram;
mod types;

// use types::*;

fn main() {
    let path = Path::new("roms/games/GUESS");

    // try to open ROM file
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}",
                           path.display(),
                           why.description()),
    };

    // convert the file from a bytestream to a vector of u16 (chip8 words)
    let bin: Vec<u16> = f.bytes()
        .map(|x| x.unwrap())
        .collect::<Vec<u8>>() // have to manually specify collection type
        .chunks(2)
        .map(|w| ((w[0] as u16) << 8) | (w[1] as u16))
        .collect();

    // init RAM
    let mut ram = ram::RAM::new();

    // Load ROM into RAM
    for (i, word) in bin.iter().enumerate() {
        if let Err(why) = ram.store_u16((0x200 + i * 2) as u16, *word) {
            println!("{}", why);
            std::process::exit(1);
        }
    }

    // init display
    let mut display = display::TermDisplay::new();

    // init CPU
    let mut cpu = cpu::CPU::new(&mut ram, &mut display);

    // Loop!
    loop {
        match cpu.cycle() {
            Ok(_) => (),
            Err(why) => {
                println!("{}", why);
                std::process::exit(1);
            }
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 100));
    }

    // // quick disasm test
    // for (i, word) in bin.iter().enumerate() {
    //     print!("0x{:03x} : 0x{:04x} : ", i * 2, word);
    //     print!("{}\n", word.clone().disasm());
    // }
}
