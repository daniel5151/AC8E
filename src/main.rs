use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod cpu;
mod disasm;
mod display;
mod ram;
mod types;

use display::Render;

fn main() {
    let path = Path::new("roms/games/MAZE");

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
    let display = display::TermDisplay::new();

    // init CPU
    let mut cpu = cpu::CPU::new(&mut ram, &display);

    // Debug Vars
    let mut debug_state = DebugState::Step;

    // Loop!
    'mainLoop: loop {
        let status = cpu.cycle();

        if let Err(why) = status {
            println!("{}", why);
            break 'mainLoop;
        }

        // TODO: All of this debug stuff should be in it's own module.

        use DebugState::*;

        debug_state = match debug_state {
            Run => Run,
            Step => Step,
            CycleFor(n) => CycleFor(n - 1),
        };

        match debug_state {
            Run => {
                display.render();
                println!("{}", cpu);
                // const SLEEP_FOR: u32 = 100000000 / 2;
                // std::thread::sleep(std::time::Duration::new(0, SLEEP_FOR));
                continue;
            }
            Step => (),
            CycleFor(0) => {
                debug_state = Step;
            }
            CycleFor(_) => continue,
        }


        display.render();
        println!("{}", cpu);

        // Handle debug input
        'debugLoop: loop {
            let mut string = String::new();
            print!("{:?}> ", debug_state);

            // TODO: error handle these bad-bois
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut string).unwrap();

            let mut words = string.split_whitespace();

            let next_debug_state: Option<DebugState> =
                match words.next() {
                    // run
                    Some("run") => Some(Run),

                    // step [num cycles]
                    Some("step") => match words.next() {
                        Some(n) => match n.trim().parse::<u32>() {
                            Ok(n) => Some(CycleFor(n)),
                            Err(_) => None,
                        },
                        None => Some(Step),
                    },

                    // exit
                    Some("exit") => break 'mainLoop,

                    // ???
                    Some(_) => None,

                    // If nothing was given, assume they just want to re-run the
                    // previous command
                    None => Some(Step),
                };

            match next_debug_state {
                Some(x) => {
                    debug_state = x;
                    break 'debugLoop;
                }
                None => println!("Invalid Command"),
            }
        }
    }
}

#[derive(Debug)]
enum DebugState {
    Step,
    Run,
    CycleFor(u32),
}
