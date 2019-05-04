use std::collections::VecDeque;
use super::opcode_instructions;

const COLUMNS: usize = 4;
const ROWS: usize = 8;

pub struct Chip8 {
    ram: [u8; 4096],
    pub stack: VecDeque<u16>,
    register: [u8; 16],
    i: u16,
    display_buffer: [u8; 32],
    pub pc: u16,
}

impl Chip8 {
    pub fn new() -> Chip8{
        Chip8 {
            ram: [0; 4096],
            stack: VecDeque::with_capacity(16),
            register: [0; 16],
            i: 0,
            display_buffer: [0; 32],
            pc: 0,
        }
    }

    pub fn write_ram(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_register(&mut self, address: u8, data: u8) {
        println!("Writing to register {:02X} with data {:02X}", address, data);
        self.register[address as usize] = data;
    }

    pub fn read_register(&self, address: u8) -> u8 {
        println!("Read register {:02X} with data {:02X}", address, self.register[address as usize]); 
        self.register[address as usize]
    }

    pub fn write_i(&mut self, data: u16) {
        self.i = data;
    }

    pub fn read_i(&self) -> u16 {
        self.i
    }

    pub fn write_to_display(&mut self, x: u8, y: u8, data: u8) {
        //Get one location
        //println!("{}", data[row * columns + column]);
        let x = self.read_register(x) as usize;
        let y = self.read_register(y) as usize;
        self.display_buffer[x * COLUMNS + y] = data;
    }

    pub fn read_display_buffer(&self, x: u8, y:u8) -> u8 {
        let x = self.read_register(x) as usize;
        let y = self.read_register(y)  as usize;
        self.display_buffer[x * COLUMNS + y]
    }

    pub fn print_display_buffer(&self) {
        let mask = 0b0000001;
        for x in 0..ROWS {
            for y in 0..COLUMNS {
                let byte = self.read_display_buffer(x as u8, y as u8);
                for i in (0..8).rev() {
                    if byte >> i  & mask == 0b1 {
                        print!("*");
                    } else {
                        print!("_");
                    }
                }
            }
            print!("\n");
    }
    }

    pub fn run(&mut self) {
        self.pc = 0x200;
        loop {
            let chunks: [u8; 2] = [self.ram[(self.pc as usize)], self.ram[(self.pc as usize + 1)]];
            opcode_instructions::run_opcode(&chunks, self);   
        }
    }
}