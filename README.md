# About
This is my attempt at writing a CHIP-8 / SCHIP interpreter. I programmed it
in rust using [SFML](http://sfml-dev.org) for handling the window creation
and graphics.

# Progress
- [x] Emulate all CHIP-8 instructions
- [x] Drawing VRAM to screen
- [x] Handling user input
- [ ] Emulate all SCHIP instructions
- [ ] Make the beep sound work

# Compiling
The only major requirement for compiling is the SFML binding for rust which can be found here
https://github.com/jeremyletang/rust-sfml

# Usage
The program uses [Clap](https://clap.rs) for command line arguments. The only required
argument is the path to the rom file you want to load. Use --help to see other options. 
