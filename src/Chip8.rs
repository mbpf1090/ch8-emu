extern crate minifb;

use std::fmt;
use std::collections::VecDeque;
use std::{thread, time};
use std::time::{Duration, Instant};
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
    keys: [u8; 16],
    tick_start: Instant,
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
            keys: [0; 16],
            tick_start: Instant::now(),
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
        self.tick_start = Instant::now();
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn delay_timer_tick(&mut self) {
        let ten_millis = Duration::from_millis(100);
        let now = Instant::now();
        if self.delay_timer > 0 && (now - self.tick_start >= ten_millis) {
            self.delay_timer -= 1;
        }
    }

    // Display
    pub fn update_window(&mut self) {
        self.window.update_with_buffer(&self.window_buffer).unwrap();
    }

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
        let mut swapped: u8 = 0b0;

        for i in 0..8 {
            let bit = (*sprite << i & 0b1000_0000) >> 7;

            if bit == 1 {
                let index = Chip8::get_index(x, (y + i) % COLUMNS as u8);
                let window_bit = (self.window_buffer[index] & 0x1) as u8;
                if window_bit == 1 {
                    swapped = 1;
                } else {
                    swapped = 0;
                }
                let pixel = bit ^ window_bit;
                self.window_buffer[index] = match pixel {
                    0 => BLACK,
                    1 => WHITE,
                    _ => unreachable!()
                };
            }
        }
        swapped
    }

    // Keyboard
    fn read_key(&mut self) {
        self.window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::Key4 => {self.set_keys(0x1)},
                    Key::Key5 => {self.set_keys(0x2)},
                    Key::Key6 => {self.set_keys(0x3)},
                    Key::Key7 => {self.set_keys(0xC)},
                    Key::R => {self.set_keys(0x4)},
                    Key::T => {self.set_keys(0x5)},
                    Key::Y => {self.set_keys(0x6)},
                    Key::U => {self.set_keys(0xD)},
                    Key::F => {self.set_keys(0x7)},
                    Key::G => {self.set_keys(0x8)},
                    Key::H => {self.set_keys(0x9)},
                    Key::J => {self.set_keys(0xE)},
                    Key::V => {self.set_keys(0xA)},
                    Key::B => {self.set_keys(0x0)},
                    Key::N => {self.set_keys(0xB)},
                    Key::M => {self.set_keys(0xB)},
                    _ => (),
                }
            }
        });
    }

    pub fn set_keys(&mut self, key: usize) {
        self.keys[key] = 1;
    }

    pub fn reset_keys(&mut self) {
        let keys = [Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::R, Key::T,
                    Key::Y, Key::U, Key::F, Key::G, Key::H, Key::J, Key::V,
                    Key::B, Key::N, Key::M];

        for key in keys.iter() {
            if !self.window.is_key_down(*key) {
                match key {
                    Key::Key4 => {self.keys[0x1] = 0},
                    Key::Key5 => {self.keys[0x2] = 0},
                    Key::Key6 => {self.keys[0x3] = 0},
                    Key::Key7 => {self.keys[0xC] = 0},
                    Key::R => {self.keys[0x4] = 0},
                    Key::T => {self.keys[0x5] = 0},
                    Key::Y => {self.keys[0x6] = 0},
                    Key::U => {self.keys[0xD] = 0},
                    Key::F => {self.keys[0x7] = 0},
                    Key::G => {self.keys[0x8] = 0},
                    Key::H => {self.keys[0x9] = 0},
                    Key::J => {self.keys[0xE] = 0},
                    Key::V => {self.keys[0xA] = 0},
                    Key::B => {self.keys[0x0] = 0},
                    Key::N => {self.keys[0xB] = 0},
                    Key::M => {self.keys[0xB] = 0},
                    _ => (),
                }
            }
        }
    }

    pub fn get_keys(&self, key: u8) -> u8 {
        self.keys[key as usize]
    }

    // Run
    pub fn run(&mut self, debug: bool) {
        
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
                    self.window.update();
                    thread::sleep(sleep_time);
                    self.reset_keys();
                }
                self.window.update();
            }
        } else {
            while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.read_key();
                let chunks: [u8; 2] = [self.ram[(self.pc as usize)], self.ram[(self.pc as usize + 1)]];
                opcode_instructions::run_opcode(&chunks, self);
                self.delay_timer_tick();
                self.window.update();
                self.reset_keys();
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
        //Header for Keys
        writeln!(f, "Keys: ")?;
        for i in 0..self.keys.len() {
            write!(f, "{:02X} ", i)?;
        }
        writeln!(f, "")?;
        //Keys
        for item in self.keys.iter() {
            write!(f, "{:02X} ", item)?;
        }
        writeln!(f, "")?;
        writeln!(f, "I {:02X}", self.i)?;
        writeln!(f, "PC: {:02X}", self.pc)?;
        writeln!(f, "Delay Timer: {}", self.get_delay_timer())
    }
}