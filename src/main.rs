use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod types;
mod disasm;
mod cpu;
mod ram;

// use types::*;

fn main() {
    let path = Path::new("roms/games/GUESS");
    let display = path.display();

    let f = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    };

    // convert the file from a bytestream to a vector of u16 (chip8 words)
    let bin: Vec<u16> = f.bytes()
        .map(|x| x.unwrap())
        .collect::<Vec<u8>>() // have to manually specify collection type
        .chunks(2)
        .map(|w| ((w[0] as u16) << 8) | (w[1] as u16))
        .collect();

    let mut ram = ram::RAM::new();

    for (i, word) in bin.iter().enumerate() {
        ram.store_u16((0x200 + i * 2) as u16, word.clone())
            .expect("poop");
    }

    let mut cpu = cpu::CPU::new(&mut ram);

    loop {
        match cpu.cycle() {
            Ok(executing) => {
                if !executing {
                    println!("Terminated!");
                    std::process::exit(0);
                }
            }
            Err(reason) => {
                println!("{}", reason);
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
