use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod cpu;
mod disasm;
mod display;
mod input;
mod ram;
mod types;

// Don't want to change the display in main, just render it...

use display::Render;
use input::Get;
use input::Set;

fn main() {
    let path = Path::new("roms/games/PONG2");

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

    // --- init RAM
    let mut ram = ram::RAM::new();

    // Before handing that RAM to the CPU, load the chosen ROM into RAM
    for (i, word) in bin.iter().enumerate() {
        if let Err(why) = ram.store_u16((0x200 + i * 2) as u16, *word) {
            println!("{}", why);
            std::process::exit(1);
        }
    }

    // --- init display
    let display = display::NcursesDisplay::new();
    display.init();

    // --- init input
    let input = input::NcursesInput::new();

    // --- init CPU,
    // the CPU takes a handle to
    //   - RAM
    //   - Display
    //   - Input
    let mut cpu = cpu::CPU::new(&mut ram, &display, &input);

    // Loop!
    'mainLoop: loop {
        // Each loop is 1/60th of a second
        std::thread::sleep_ms(16); // ~ 1/60

        // Run the CPU faster than the screen refreshes
        for _ in 0..20 {
            // Run the cpu, and get it's state
            let cpu_state = match cpu.cycle() {
                // Shutdown everything if shit hits the fan
                Err(why) => {
                    println!("{}", why);
                    break 'mainLoop;
                }
                Ok(state) => state,
            };

            // Check input, possibly blocking execution
            input.update_keys(cpu_state == cpu::CPUState::WaitForInput);

            if input.pressed_esc() {
                break 'mainLoop;
            }
        }

        // Decrement the time-based registers
        cpu.decrement_st();
        cpu.decrement_dt();

        // ...
        input.decrement_keys();

        // Render the screen
        display.render();

        // Incredibly dirty debug...
        extern crate ncurses;
        use types::Chip8Utils;
        use std;

        ncurses::mv(33, 0);

        // Print some ram
        for dpc in -3i32..4i32 {
            ncurses::printw(format!("{} {:#03x} : {}\n",
                                    if dpc == 0 { "> " } else { "  " },
                                    cpu.pc as i32 + dpc * 2,
                                    cpu.ram
                                        .load_u16((cpu.pc as i32 + dpc * 2) as
                                                  u16)
                                        .unwrap()
                                        .disasm())
                                    .as_ref());

        }


        // Print out the CPU state
        ncurses::printw(format!("Cycle: {}\n\n", cpu.cycle).as_ref());

        let instr = cpu.ram.load_u16(cpu.pc).unwrap();
        ncurses::printw(format!("[{:#03x}] {}\n\n", cpu.pc, instr.disasm())
                            .as_ref());

        for (i, r) in cpu.v.iter().enumerate() {
            ncurses::printw(format!("V{:x}: {:<3} (0x{:02x})  ", i, r, r)
                                .as_ref());
            if (i + 1) % 4 == 0 {
                ncurses::printw("\n");
            }
        }

        ncurses::printw("\n");
        ncurses::printw(format!(" I: 0x{:03x} ", cpu.i).as_ref());
        ncurses::printw(format!(" DT: {:02x} ", cpu.dt).as_ref());
        ncurses::printw(format!(" ST: {:02x} ", cpu.st).as_ref());

        ncurses::printw(format!(" Stack: {:?}\n", cpu.stack).as_ref());

        ncurses::printw("\n");

        // Keypad
        for c in ['1', '2', '3', '4', 'q', 'w', 'e', 'r', 'a', 's', 'd', 'f',
                  'z', 'x', 'c', 'v']
                    .iter() {
            let i = match *c {
                '1' => 0x1,
                '2' => 0x2,
                '3' => 0x3,
                '4' => 0xC,
                'q' => 0x4,
                'w' => 0x5,
                'e' => 0x6,
                'r' => 0xD,
                'a' => 0x7,
                's' => 0x8,
                'd' => 0x9,
                'f' => 0xE,
                'z' => 0xA,
                'x' => 0x0,
                'c' => 0xB,
                'v' => 0xF,
                _ => return,
            };

            let val = cpu.input.keys.borrow()[i as usize];

            ncurses::printw(format!("{:3} ", val).as_ref());

            if *c == '4' || *c == 'r' || *c == 'f' || *c == 'v' {
                ncurses::printw("\n");
            }
        }

        // ncurses::timeout(-1);
        // match ncurses::getch() {
        //     ncurses::KEY_F1 => {
        //         display.uninit();
        //         return;
        //     }
        //     _ => (),
        // };

    }

    display.uninit();
}
