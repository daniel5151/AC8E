## AC8E, a retrospective

With AC8E finally working properly, and (as far as I can tell) accurately
emulating the CHIP-8, I think it would be good to look back on the project, and
reflect on what I think I did well, and what could have been done better.

### The Good

#### It works!

I'm ***SUPER*** happy that AC8E ended up working in the end :smile:

I am confident that when I start working on my next emulator, I can think back
to all the good (and bad) things that I did in AC8E, and have development go a
bit more smoothly.

#### Rust itself

I started AC8E with no knowledge of Rust whatsoever, only knowing that people
(\**cough*\* hackernews \**cough*\*) thought Rust was amazing. Well, I wanted to
see fi the hype was real, so I decided to dip my feet in the water.

I get the hype now.

*Rust is rad.*

The thing that excites me most about using Rust isn't really the borrow-checker,
or it's memory safety guarantees (although don't get me wrong, never having to
see that dreaded Segfault message was super amazing), it was Rust's high-level,
zero cost abstractions.

Iterators in Rust can be mapped over, reduced over, and a whole lot more. If you
look around AC8E's source, you'll find these little tidbits of functional code
that look beautiful, and more importantly, perform almost as well as their
imperative counterparts! Writing functional code in a systems language made me
feel all warm and fuzzy, and it was one of my major highlights of working with
Rust.

Also, kudos to `rustup` and `cargo` for making installing and working with Rust
a breeze. It's refreshing to see a low level language have such polished and
user friendly tools for installation and dependency management. Heck, one can
compare using `cargo`to using `npm`, it's that easy!

#### `CPU.cycle()`

This method is :100:

- It's a prime example of the power and expressiveness of Rust's amazing
  pattern-matching.
- The code itself if pretty DRY (compared to some other CHIP-8 emulators i've
  come across)
  - Eg: the opcode is decomposed at the top of `CPU.cycle()`, making variables
        like `nnn`, `kk`, `x`, `y`, etc... easy to access
  - Eg: `self.pc` is incremented right after it fetched, so individual op-codes
        do not have to worry about incrementing the PC.
- Error propagation is nice and clean, with IMHO good use of `Result` and `?`
  to keep the code nice and readable.
- Of all the ways `0xFx0A` (halt execution until a key is pressed) could be
  implemented, I think my method of returning a `CPUState` enum is pretty solid.

#### My makeshift disassembler

Like `CPU.cycle()`, the disassembler is a beautiful example of Rust's pattern
matching. I just *really* like how clean the disassembler is.

#### Extending `u16` with `.nibble_at()`

- The fact that I could extend a built-in type with a new method simply by
  defining a trait, and implementing it, is really, really rad!
  - using `.nibble_at()` feels natural, almost as it it was a built in method!

---

### The Ugly

#### Oh Borrow Checker...

I'm not the first to say this, and I'm sure I won't be the last to say this, but
damn, coming from a C++ background, is it hard to write Rust code that
satisfies the Borrow Checker.

See, even though Rust and C++ are Systems languages with *similar* syntaxes, the
code-style that Rust promotes is totally different from the code style that C++
enables.

The major mistake that I made when starting AC8E was not fully understanding how
Rust Traits are a very different beast from C++ Classes. Indeed, my biggest
blunder was...

#### Display / Input Interfaces

Seperating the Display / Input interfaces was silly. Splitting each interface
into a "CPU facing" interface, and a "Main facing" interface was sillier.

I was *hoping* that by splitting everything up, it would be easy to swap-out
interfaces for one another, and in a sense, it did... as long as you don't mind
recompiling every time you want to switch interfaces.

I was hoping I could do something like pass in a CLI flag, and conditionally
initialize a display / input "class" to pass to the CPU, but alas, the way I
implemented things was not very compatible with the way Traits work.

I'm not going to refactor the interfaces now, but if I were to revisit AC8E in
the future, that would be the first thing on my TODO list.

---

Lastly...

#### I HATE BINARY CODED DECIMAL

I COULDN'T GET _PONG_ WORKING FOR 3 WEEKS BECAUSE I DIDN'T REALIZE I
IMPLEMENTED BCD WRONG! AAARRGGHHH!
