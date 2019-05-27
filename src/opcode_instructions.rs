use super::chip8;
use rand::Rng;

pub fn run_opcode(chunk: &[u8], chip8: &mut chip8::Chip8){
        let mut rng = rand::thread_rng();
        if chunk[0] == 0 && chunk[1] == 0 {
                //println!("EMPTY");
        }
            let nnn = (((chunk[0] as u16) << 8) | (chunk[1] as u16)) & 0x0FFF;
            let n = chunk[1] & 0x0F;
            let x = chunk[0] & 0x0F;
            let y = chunk[1] >> 4;
            let kk = chunk[1];

            match chunk[0] >> 4 {
                0x0 => match chunk[0] >> 3 {
                        0x0 => match chunk[1] & 0x0F {
                                0xE => {
                                        //println!("Returning from stack");
                                        chip8.pc = chip8.stack.pop_front().unwrap();
                                        },
                                0x0 => {
                                        //println!("Clear Display");
                                        chip8.clear_window();
                                        chip8.pc += 2;     
                                },
                                _ => {unreachable!()}
                        },
                        _ => {
                                panic!("Jump to machine code routine at ram {:02X}", nnn);
                                chip8.pc = nnn;
                        },
                },
                0x1 => {
                                chip8.pc = nnn;
                                //println!("Jump to {:0X}", nnn);
                },
                0x2 => {
                                //println!("CALL {:02X}", nnn);
                                // Better not increment here chip8.pc += 2;
                                chip8.stack.push_front(chip8.pc + 2);
                                chip8.pc = nnn;
                },
                0x3 => {
                        //println!("Skip next instruction if register {:02X} with value {} = {}", x, chip8.read_register(x), kk);
                        if chip8.read_register(x) == kk {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0x4 => {
                        //println!("Skip next instruction if register {:02X} with value {} != {}", x, chip8.read_register(x), kk);
                        if chip8.read_register(x) != kk {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0x5 => {
                        //println!("Skip next instruction if register {:02X} with value {} == {}", x, chip8.read_register(x), chip8.read_register(y));
                        if chip8.read_register(x) == chip8.read_register(y) {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0x6 => {
                        chip8.write_register(x, kk);
                        chip8.pc += 2;
                        //println!("Write into register {:02X} value {:02X}", x, kk);
                },
                0x7 => {
                        let (data, _) = chip8.read_register(x).overflowing_add(kk);
                        //println!("Set V{:02X} = {} + {}.", x, chip8.read_register(x), kk);
                        chip8.write_register(x, data);
                        chip8.pc += 2;
                },
                0x8 => match chunk[1] & 0x0F {
                        0x0 => {
                                let y = chip8.read_register(y);
                                chip8.write_register(x, y);
                                //println!("Set register {:02X} to {:02X}", x, y);
                                chip8.pc += 2;
                                },
                        0x1 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let data = x_value | y_value;
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                                //println!("Set register {:02X} to {:08b} = {:08b} OR {:08b}.", x, data, x_value, y_value);
                        },
                        0x2 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let data = x_value & y_value;
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                                //println!("Set register {:02X} to {:08b} = {:08b} AND {:08b}.", x, data, x_value, y_value);
                        },
                        0x3 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let data = x_value ^ y_value;
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                                //println!("Set register {:02X} to {:08b} = {:08b} XOR {:08b}.", x, data, x_value, y_value);
                        },
                        0x4 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let (data, overflow) = x_value.overflowing_add(y_value);
                                //println!("Set register {:02X} to value {:02X} = {:02X} + {:02X}", x, data, x_value, y_value);
                                chip8.write_register(x, data);
                                match overflow {
                                        true => chip8.write_register(0xF, 1),
                                        false => chip8.write_register(0xF, 0),
                                }
                                chip8.pc += 2;
                        },
                        0x5 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let (data, overflow) = x_value.overflowing_sub(y_value);
                                match overflow {
                                        true => chip8.write_register(0xF, 0),
                                        false => chip8.write_register(0xF, 1),
                                }
                                //println!("Set register {:02X} to value {:02X} = {:02X} - {:02X}", x, data, x_value, y_value);
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                        },
                        0x6 => {
                                let data = chip8.read_register(x); 
                                chip8.write_register(0xF, data & 0b0000_0001);
                                chip8.write_register(x, data >> 1);
                                chip8.pc += 2
                        },
                        0x7 => panic!("unimplemented"),
                        0xE => {
                                let data = chip8.read_register(x); 
                                chip8.write_register(0xF, data & 0b1000_0000);
                                chip8.write_register(x, data << 1);
                                chip8.pc += 2
                        },
                        _ => panic!("0x8 ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                },
                0x9 => {
                        //println!("Skip next instruction if register {:02X} with value {} != {}", x, chip8.read_register(x), chip8.read_register(y));
                        if chip8.read_register(x) != chip8.read_register(y) {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0xA => {
                        chip8.write_i(nnn);
                        chip8.pc += 2;
                        //println!("Set I to {:02X}", nnn);
                        },
                0xB => {
                        //println!("Jump to {} + V0: {} => {}", nnn, chip8.read_register(0x0), nnn + chip8.read_register(x) as u16);
                        chip8.pc = nnn + chip8.read_register(0x0) as u16;
                },
                0xC => {
                        //println!("generate random number to register {:02X}", x);
                        let rnd = rng.gen_range(0, 255);
                        chip8.write_register(x, rnd & kk);
                        chip8.pc += 2;
                },
                0xD => {
                                let i = chip8.read_i();
                                let x = chip8.read_register(x);
                                let mut y = chip8.read_register(y);
                                let mut swapped: u8 = 0b0;
                                for address in i..i + n as u16 {
                                        let sprite = chip8.read_ram(address);
                                        let local_swapped = chip8.write_sprite_to_window(&sprite, y, x);
                                        if local_swapped == 1 {
                                                swapped = 0b1;
                                        }
                                        y = (y + 1) % 32;
                                }
                                if swapped == 1 {
                                        chip8.write_register(0xF, 1);
                                } else {
                                        chip8.write_register(0xF, 0);
                                }
                                chip8.update_window();
                                chip8.pc += 2;
                        },
                0xE => match chunk[1] {
                        0x9E => {
                                //println!("Skip next instruction if key in register {} with value of {} is pressed", x, chip8.read_register(x));
                                //println!("{:?}", chip8);
                                let key = chip8.read_register(x);
                                //println!("Key: pressed {}", key);
                                if chip8.get_keys(key) == 1 {
                                        chip8.pc += 4;
                                } else {
                                        chip8.pc += 2;
                                }
                                chip8.reset_keys();
                        },
                        0xA1 => {
                                //println!("Keyboard function not implemented");
                                //println!("Skip next instruction if key in register {} with value of {} not pressed", x, chip8.read_register(x));
                                //println!("{:?}", chip8);
                                let key = chip8.read_register(x);
                                //println!("Key: pressed {}", key);
                                if chip8.get_keys(key) != 1 {
                                        chip8.pc += 4;
                                } else {
                                        chip8.pc += 2;
                                }
                                chip8.reset_keys();
                        },
                        _ => {
                                panic!("0xE Error {:02X} {:02X}", chunk[0], chunk[1]);
                                },
                },
                0xF => match chunk[1] {
                        0x07 => {
                                //println!("Writing delay timer {} to register {:02X}", chip8.get_delay_timer(), x);
                                chip8.write_register(x, chip8.get_delay_timer());
                                chip8.pc += 2;
                        },
                        0x0A => {
                                //println!("Blocking key");
                                if let Some(key) = chip8.read_all_keys() {
                                                chip8.write_register(x, key);
                                                chip8.pc += 2;
                                } else {
                                        chip8.pc += 0;
                                }
                        },
                        0x15 => {
                                // set delay timer
                                chip8.set_delay_timer(chip8.read_register(x));
                                //println!("Delay timer seet to: {}", chip8.read_register(x));
                                chip8.pc += 2;
                        },
                        0x18 => {
                                //println!("Sound unimplemented");
                                chip8.pc += 2;
                        },
                        0x1E => {
                                let i = chip8.read_i();
                                let x = chip8.read_register(x) as u16;
                                //println!("Set I to {}", i + x);
                                chip8.write_i(i + x);
                                chip8.pc += 2;
                        
                        },
                        0x29 => {
                                //println!("Set I to font adress of {:02X}", x);
                                let character = chip8.read_register(x);
                                chip8.write_i(character as u16 * 5);
                                chip8.pc += 2;
                        },
                        0x33 => {       
                                        let digit = chip8.read_register(x);
                                        let s = (digit / 10_u8.pow(0)) % 10;
                                        let t = (digit / 10_u8.pow(1)) % 10;
                                        let h = (digit / 10_u8.pow(2)) % 10;
                                        //println!("BCD: {} = {} {} {}", digit, h, t, s);
                                        chip8.write_ram(chip8.read_i(), h);
                                        chip8.write_ram(chip8.read_i() + 1, t);
                                        chip8.write_ram(chip8.read_i() + 2, s);
                                        chip8.pc += 2;
                        },
                        0x55 => {
                                        //Store registers V0 through Vx in memory starting at location I.
                                        for i in 0..=x {
                                                chip8.write_ram(chip8.read_i() + i as u16, chip8.read_register(i));
                                        }
                                        chip8.pc += 2;
                        },
                        0x65 => {       

                                        let start_address = chip8.read_i();
                                        //println!("Write to registers 0 through {:02X} from memory starting at location {:02X}.", x, start_address);
                                        for i in 0..=x {
                                                let data = chip8.read_ram(start_address + i as u16);
                                                //println!("Write to register {} with data {}", x, data);
                                                chip8.write_register(i, data);
                                        }
                                        // HMM?!
                                        //chip8.write_i(start_address + x as u16 + 1);
                                        chip8.pc += 2;
                        },
                        _ => {
                                panic!("0xF ERROR {:02X} {:02X}", chunk[0], chunk[1]);
                        },
                },
                _ => panic!("Opcode not found"),
            };
}