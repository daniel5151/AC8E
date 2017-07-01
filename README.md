# AC8E - Another CHIP-8 Emulator
### A CHIP-8 emulator written in Rust for fun and learning

I've never used Rust before, so this project is my way of trying to get a feel
for the language.

If this code isn't :100: *idiomatic* Rust, cut me some slack :smile:

#### Update:

At this point, AC8E is working, and is (as far as I can tell) accurate. It's
definately not the best CHIP-8 emulator out there, but it's mine, and hey,
that's pretty cool.

I might revisit AC8E, refactor it a bit, add a non-terminal based renderer
(SDL?), but for now, i'm just going to let AC8E sit.

Check out **THOUGHTS.md** if you'd like to read my reflections on the project.

### How To Run

`cargo build --release` for a nice, optimized executable. Then, just `./target/
release/ac8e [path to rom]`.

`cargo run [path to rom]` for a more quick-and-dirty run.

### External Dependencies

Make sure `ncurses` is installed. If you're on Linux / OSX, this should be
a relatively straightforward process.

If you're on Windows, glhf.

### How To Switch Renderers

I haven't gotten around to figuring out the right way to switch renderers based
on command line arguments, but if you'd like to try out the other renderers,
you have to manually modify the `let display = ...` and `let input = ...` lines
in `main.rs` to point to the appropriate Display / Input implementations.

Right now, the following interfaces are implemented:

- _Null Renderer / Null Input_
  - Does nothing, only used for testing
- _Terminal Renderer / Null Input_
  - Renders line-by-line to the terminal
    - _Cripplingly slow_ (good enough for basic ROMs - eg: `MAZE`)
  - No interactivity
- **Ncurses Renderer / Ncurses Input**
  - **Default Renderer**
  - Uses `ncurses-rs` to render the display, and get user input
  - **NOTE:** Requires `ncurses` to be installed as a system library
