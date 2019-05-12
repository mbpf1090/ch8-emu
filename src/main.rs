use std::fs::File;
use std::io::{Read, Error};
use std::env;

mod opcode_instructions;
mod chip8;
mod font;

fn read_rom(file_path: &str) -> Result<Vec<u8>, Error> {
    let mut f = File::open(file_path)?;
    let mut buff = vec!();
    let _result = f.read_to_end(&mut buff)?;
    Ok(buff)
}



fn main() {

    // get args
    let mut args = env::args();
    args.next();

    let path = match args.next() {
        Some(path) => path,
        None => panic!("Please provide a valid path to a CHIP-8 rom!")
     };


    // open file
    let rom = match read_rom(&path) {
        Ok(rom) => rom,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };

    let mut chip8 = chip8::Chip8::new();

    // load fonts
    let fonts = font::Fonts::new();
    for (index, data) in fonts.fonts.iter().enumerate() {
        chip8.write_ram(index as u16, *data);
    }

    //load rom into memory
    for (index, item) in rom.iter().enumerate() {
        chip8.write_ram(0x200 + index as u16,  *item);
    }  

    // run instructions
    chip8.run()
}
