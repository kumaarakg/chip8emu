# Chip 8 emulator

A CHIP-8 emulator in Rust is a software program that mimics the behavior of the CHIP-8, a simple, interpreted programming language originally developed in the 1970s for programming calculators and early home computers. Emulators for CHIP-8 can run classic games like Tetris and Space Invaders by interpreting the CHIP-8 instructions and managing the virtual machine's state, including memory, registers, and input/output operations. Implementing a CHIP-8 emulator in Rust involves leveraging Rust's performance and safety features, such as its strong type system and memory safety guarantees, to create an efficient and reliable emulator. The project typically involves parsing CHIP-8 opcodes, implementing the corresponding instructions, handling input from the user, and rendering graphics using a library such as minifb for display.

## Resources
* [Basic info and op codes](https://en.wikipedia.org/wiki/CHIP-8)
* [additional info](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
* [Rust](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

## Requirements
 Make sure you have Rust and Cargo installed

## Usage

clone this repository and run the command

```
cargo run
```

You can change the ROM by changing the path in the main file



