use std::env;
use std::fs::File;
use std::io::Read;

mod cpu;
mod disasm;
mod display;
mod input;
mod ram;
mod types;

use display::Render;
use input::Get;
use input::Set;

fn main() {
    // read ROM path from cli
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: ac8e <romfile>");
            std::process::exit(1);
        }
    };

    // try to open ROM file
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("couldn't find '{}'", path);
            std::process::exit(1);
        }
    };

    // --- init RAM
    let mut ram = ram::RAM::new();

    // Load the rom file into RAM (before handing RAM to CPU)
    for (i, byte) in f.bytes().enumerate() {
        // make sure the byte read correctly
        let byte = match byte {
            Ok(byte) => byte,
            Err(_) => {
                println!("couldn't read '{}'", path);
                std::process::exit(1);
            }
        };

        // And make sure it is loaded into RAM properly too
        if let Err(why) = ram.store_u8(0x200 + i as u16, byte) {
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
        // Each loop is ~ 1/120th of a second
        std::thread::sleep(std::time::Duration::from_millis(8));

        // Run the CPU faster than the screen refreshes
        for _ in 0..5 {
            // Run the cpu, and get it's state
            let cpu_state = match cpu.cycle() {
                // Shutdown everything if shit hits the fan
                Err(why) => {
                    display.uninit();
                    print!("\n{}\n", why);
                    break 'mainLoop;
                }
                Ok(state) => state,
            };

            match cpu_state {
                cpu::CPUState::WaitForInput => {
                    display.render(); // render the screen before blocking
                    input.update_keys(true); // blocking operation
                }
                cpu::CPUState::Running => {
                    input.update_keys(false);
                }
            };

            // check if user wants to exit
            if input.pressed_esc() {
                break 'mainLoop;
            }
        }

        // Decrement the time-based registers
        cpu.decrement_counters();

        // ...
        input.decrement_keys();

        // Render the screen
        display.render();
    }

    display.uninit();
}
