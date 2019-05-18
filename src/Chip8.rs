extern crate minifb;

use std::fmt;
use std::collections::VecDeque;
use std::{thread, time};
use super::opcode_instructions;

use minifb::{Key, WindowOptions, Window, Scale, KeyRepeat};

const COLUMNS: usize        = 64;
const ROWS: usize           = 32;
const SLEEP_TIME: u64       = 1;
const RAM_SIZE: usize       = 4096;
const REGISTER_SIZE: usize  = 16;
const STACK_SIZE: usize     = 24;
const PROGRAMM_START: u16   = 0x200;
const WHITE: u32            = 0xFFFFFF;
const BLACK: u32            = 0x000000;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    pub stack: VecDeque<u16>,
    register: [u8; REGISTER_SIZE],
    i: u16,
    window_buffer: Vec<u32>,
    window: Window,
    pub pc: u16,
    delay_timer: u8,
    key: u8,
}

impl Chip8 {
    pub fn new() -> Chip8{
        let win = Window::new("Chip-8 - ESC to exit",
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
            window_buffer: vec![0; COLUMNS * ROWS],
            window: win,
            pc: 0,
            delay_timer: 0,
            key: 0xFF_u8,
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

    // Display
    pub fn clear_window(&mut self) {
        for pixel in 0..self.window_buffer.len() {
            self.window_buffer[pixel] = 0;
        }
         self.window.update_with_buffer(&self.window_buffer).unwrap();
    }

    fn get_index(x: u8, y: u8) -> usize {
        x as usize * COLUMNS + y as usize
    }

    pub fn write_sprite_to_window(&mut self, sprite: &u8, x: u8, y: u8) -> u8{
        let mask = 0b000_0001;
        let mut swapped: u8 = 0b0;
        let x = x % ROWS as u8;
        let y = y % COLUMNS as u8;



        for i in 0..8 {
            let bit = (*sprite << i & 0b1000_0000) >> 7;
            //let foo = (sprite >> i & mask) as u32;
            //if y + i >= COLUMNS as u8 {
            //    continue;
            //}
            let index = Chip8::get_index(x, y + i);
            //let index = Chip8::get_index(x, y + 7 - i);
            let window_bit = (self.window_buffer[index] & 0x1) as u8;
            //println!("Drawing bit {} at x {} y {} with widnow bit set to {}", bit, x, y + 7 - i, window_bit);


            if (window_bit == 1) && (bit == 1) {
                swapped = 0b1;
                //self.write_register(0xF, swapped);
            }
            if bit != 0_u8 {
                let pixel: u8 = bit ^ window_bit;
                self.window_buffer[index] = match pixel {
                    0 => BLACK,
                    1 => WHITE,
                    _ => unreachable!()
                };

            }
        }
        self.window.update_with_buffer(&self.window_buffer).unwrap();
        swapped
    }

    // Keyboard
    fn read_key(&mut self) {
        self.window.get_keys_pressed(KeyRepeat::No).map(|keys| {
        //self.window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Key4 => {self.key = 0x1},
                    Key::Key5 => {self.key = 0x2},
                    Key::Key6 => {self.key = 0x3},
                    Key::Key7 => {self.key = 0xC},
                    Key::R => {self.key = 0x4},
                    Key::T => {self.key = 0x5},
                    Key::Y => {self.key = 0x6},
                    Key::U => {self.key = 0xD},
                    Key::F => {self.key = 0x7},
                    Key::G => {self.key = 0x8},
                    Key::H => {self.key = 0x9},
                    Key::J => {self.key = 0xE},
                    Key::V => {self.key = 0xA},
                    Key::B => {self.key = 0x0},
                    Key::N => {self.key = 0xB},
                    Key::M => {self.key = 0xF},
                    _ => {self.key = 0xFF},
                }
            }
        });
    }

    pub fn get_key(&self) -> u8 {
        self.key
    }

    pub fn reset_key(&mut self) {
        self.key = 0xFF;
    }

    // Run
    pub fn run(&mut self) {
        let debug = false;
        let sleep_time = time::Duration::from_millis(SLEEP_TIME);
        self.pc = PROGRAMM_START;
        if debug {
            while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                if self.window.is_key_pressed(Key::W, KeyRepeat::Yes) {
                    self.read_key();
                    let chunks: [u8; 2] = [self.ram[(self.pc as usize)], self.ram[(self.pc as usize + 1)]];
                    opcode_instructions::run_opcode(&chunks, self);
                    self.delay_timer_tick();
                    println!("{:?}", self);
                    thread::sleep(sleep_time);
                }
                self.window.update_with_buffer(&self.window_buffer).unwrap();
            }
        } else {
            while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.read_key();
                let chunks: [u8; 2] = [self.ram[(self.pc as usize)], self.ram[(self.pc as usize + 1)]];
                opcode_instructions::run_opcode(&chunks, self);
                self.delay_timer_tick();
                thread::sleep(sleep_time);
                self.window.update();
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