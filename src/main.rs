
use std::{fs::File, io::Read};
use chip8::Chip8;
mod ram;
mod chip8;
mod cpu;


fn main(){
    let mut file = File::open("data/BLITZ").unwrap();

    let mut data=Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8=Chip8::new();
    chip8.load_rom(&data);
    
    loop {
        chip8.run_instruction();
    }

}