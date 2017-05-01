# DMGR - Rust Gameboy Emulator

## About
This is the final project I made for programming studio.
I decided to make an accurate gameboy original (DMG) emulator.

## Usage
Compile with cargo.  Simply execute `cargo run <rom file>` to have the emulator
boot up.

## Dependencies
You need a valid installation of rust and cargo.  Cargo will pull in the SDL
dependencies automatically.

## References

Some resources I used when developing this project.
* [Official Gameboy Programming Manual](http://www.chrisantonellis.com/files/gameboy/gb-programming-manual.pdf)
* [Pandocs - Hardware reference](http://problemkaputt.de/pandocs.htm)
* [GBCPU Unofficial Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
* [Opcode chart](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Boot rom disassembly](http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM)
* [More hardware](http://www.devrs.com/gb/hardware.php#hardgb)

Tutorials
* [Wornwinter Series](https://wornwinter.wordpress.com/2015/02/05/adventures-in-gameboy-emulation-part-1/)
* [Realboy Series](https://realboyemulator.wordpress.com/2013/01/02/the-nintendo-game-boy-part-2/gbcpuman/)
* [Javascript Series](http://imrannazar.com/Gameboy-Z80-Opcode-Map)

Other emulators
* [BGB - Great Reference!](http://bgb.bircd.org/)
* [Cinoop - C gameboy emulator](https://github.com/CTurt/Cinoop/)
* [Another Rust gameboy emulator](https://github.com/mvdnes/rboy)

Library References
* [Rust SDL2 Tutorial](http://jadpole.github.io/arcaders/arcaders-1-1)

Interesting Links
* [Rust on GBA](http://csclub.uwaterloo.ca/~tbelaire/blog/posts/gba-rust-1.html)
* [Nes emulator](https://github.com/fogleman/nes)
* [Nes tutorial series](https://medium.com/@fogleman/i-made-an-nes-emulator-here-s-what-i-learned-about-the-original-nintendo-2e078c9b28fe)
