extern crate minifb;

use std::fmt;
use std::collections::VecDeque;
use std::{thread, time};
use super::opcode_instructions;

use minifb::{Key, WindowOptions, Window, Scale, KeyRepeat};

const COLUMNS: usize = 64;
const ROWS: usize = 32;
const SLEEP_TIME: u64 = 1;
const RAM_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 24;

pub struct Chip8 {
    ram: [u8; 4096],
    pub stack: VecDeque<u16>,
    register: [u8; 16],
    i: u16,
    display_buffer: [u8; ROWS * COLUMNS],
    window_buffer: Vec<u32>,
    window: Window,
    pub pc: u16,
    delay_timer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8{
        let win = Window::new("Test - ESC to exit",
                                 COLUMNS,
                                 ROWS,
                                 WindowOptions {
                                     resize: false,
                                     scale: Scale::X4,
                                     ..WindowOptions::default()
                        }).expect("Unable to Open Window");
        Chip8 {
            ram: [0; RAM_SIZE],
            stack: VecDeque::with_capacity(STACK_SIZE),
            register: [0; REGISTER_SIZE],
            i: 0,
            display_buffer: [0; ROWS * COLUMNS],
            window_buffer: vec![0; COLUMNS * ROWS],
            window: win,
            pc: 0,
            delay_timer: 0,
        }
    }


    // RAM
    pub fn write_ram(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }


    // Registers
    pub fn write_register(&mut self, address: u8, data: u8) {
        self.register[address as usize] = data;
    }

    pub fn read_register(&self, address: u8) -> u8 {
        self.register[address as usize]
    }

    // Register I
    pub fn write_i(&mut self, data: u16) {
        self.i = data;
    }

    pub fn read_i(&self) -> u16 {
        self.i
    }


    // Delay Timer
    pub fn set_delay_timer(&mut self, delay: u8) {
        self.delay_timer = delay;
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn delay_timer_tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    // Debug Display
    pub fn write_sprite(&mut self, sprite: &u8, x: u8, y: u8) {
        let columns: usize = 64;
        let mask = 0b0000001;
        for i in (0..8) {
            if sprite >> i  & mask == 0b1 {
                self.display_buffer[(x as usize) * columns + (y as usize + i)] = 1;
            } else {
                self.display_buffer[(x as usize) * columns + (y as usize + i)] = 0;
            }
        }
    }


    pub fn read_display_buffer(&self, x: u8, y:u8) -> u8 {
        let x = self.read_register(x) as usize;
        let y = self.read_register(y)  as usize;
        self.display_buffer[x * COLUMNS + y]
    }

    pub fn print_display_buffer(&self) {
        for x in 0..ROWS {
            for y in 0..COLUMNS {
                let bit = self.display_buffer[x * COLUMNS + y];
                if bit == 0b1 {
                    print!("*");
                } else {
                    print!("_");
                }
            }
            print!("\n");        
        }
    }


    // Display
    pub fn clear_display(&mut self) {
        for pixel in 0..self.display_buffer.len() {
            self.display_buffer[pixel] = 0;
        }
    }

    pub fn clear_window(&mut self) {
        for pixel in 0..self.window_buffer.len() {
            self.window_buffer[pixel] = 0;
        }
    }

    pub fn write_sprite_to_window(&mut self, sprite: &u8, x: u8, y: u8) {
        let mask = 0b0000001;
        let mut swapped: u8 = 0b0;
        
        for i in (0..8).rev() {
            let bit = (sprite >> i & mask) as u32;
            let window_bit = self.window_buffer[(x as usize) * COLUMNS + (y as usize + 7 - i)] as u32;
            
            if (bit == 1) & (window_bit == 1) {
                self.window_buffer[(x as usize) * COLUMNS + (y as usize + 7 - i)] = 0x000000;
                swapped = 0b1;
            } else if bit == 1 {
                self.window_buffer[(x as usize) * COLUMNS + (y as usize + 7 - i)] = 0xFFFFFF;
            } else {
                self.window_buffer[(x as usize) * COLUMNS + (y as usize + 7 - i)] = 0x000000;
            }
        }
        self.write_register(0xF, swapped);
    }

    pub fn get_key(&self) -> u8 {
        let mut key = 0_u8;
        self.window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Key4 => {key = 0x1},
                    Key::Key5 => {key = 0x2},
                    Key::Key6 => {key = 0x3},
                    Key::Key7 => {key = 0xC},
                    Key::R => {key = 0x4},
                    Key::T => {key = 0x5},
                    Key::Z => {key = 0x6},
                    Key::U => {key = 0xD},
                    Key::F => {key = 0x7},
                    Key::G => {key = 0x8},
                    Key::H => {key = 0x9},
                    Key::J => {key = 0xE},
                    Key::V => {key = 0xA},
                    Key::B => {key = 0x0},
                    Key::N => {key = 0xB},
                    Key::M => {key = 0xF},
                    _ => (),
                }
            }
        });
        key
    }

    // Run
    pub fn run(&mut self) {
        let debug = false;
        let sleep_time = time::Duration::from_millis(SLEEP_TIME);
        self.pc = 0x200;
        if debug {
            while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.window.update_with_buffer(&self.window_buffer).unwrap();
                if self.window.is_key_pressed(Key::W, KeyRepeat::Yes) {
                    let chunks: [u8; 2] = [self.ram[(self.pc as usize)], self.ram[(self.pc as usize + 1)]];
                    opcode_instructions::run_opcode(&chunks, self);
                    self.delay_timer_tick();
                    println!("{:?}", self);
                    thread::sleep(sleep_time);
                }
            }
        } else {
            while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.window.update_with_buffer(&self.window_buffer).unwrap();
                
                    let chunks: [u8; 2] = [self.ram[(self.pc as usize)], self.ram[(self.pc as usize + 1)]];
                    opcode_instructions::run_opcode(&chunks, self);
                    self.delay_timer_tick();
                    thread::sleep(sleep_time);
            }
        }
    }

}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "*****Debugger*****")?;
        // Header for Registers
        writeln!(f, "Registers: ")?;
        for i in 0..self.register.len() {
            write!(f, "{:02X} ", i)?;
        }
        writeln!(f, "")?;
        //Registers
        for item in self.register.iter() {
            write!(f, "{:02X} ", item)?;
        }
        writeln!(f, "")?;
        writeln!(f, "I {:02X}", self.i)?;
        writeln!(f, "PC: {:02X}", self.pc)?;
        writeln!(f, "Delay Timer: {}", self.get_delay_timer())
    }
}