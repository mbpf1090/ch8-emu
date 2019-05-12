use super::chip8;

pub fn run_opcode(chunk: &[u8], chip8: &mut chip8::Chip8){

    if chunk[0] == 0 && chunk[1] == 0 {
                println!("EMPTY");
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
                                        println!("Returning from stack");
                                        chip8.pc = chip8.stack.pop_front().unwrap() + 2;
                                        },
                                0x0 => {
                                        println!("Clear Display");
                                        chip8.clear_window();
                                        chip8.pc += 2;     
                                },
                                _ => {
                                        println!("ERROR {:02X} {:02X}", chunk[0], chunk[1]);
                                        panic!("Error");
                                },
                        },
                        _ => {
                                println!("Jump to machine code routine at ram {:02X}", nnn);
                                chip8.pc = nnn;
                        },
                },
                0x1 => {
                                chip8.pc = nnn;
                                println!("Jump to {:0X}", nnn)
                },
                0x2 => {
                                println!("CALL {:02X}", nnn);
                                // Better not increment here chip8.pc += 2;
                                chip8.stack.push_back(chip8.pc);
                                chip8.pc = nnn;
                },
                0x3 => {
                        println!("Skip next instruction if register {:02X} with value {} = {}", x, chip8.read_register(x), kk);
                        if chip8.read_register(x) == kk {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0x4 => {
                        println!("Skip next instruction if register {:02X} with value {} != {}", x, chip8.read_register(x), kk);
                        if chip8.read_register(x) != kk {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0x5 => {
                        println!("Skip next instruction if register {:02X} with value {} == {}", x, chip8.read_register(x), chip8.read_register(y));
                        if chip8.read_register(x) == chip8.read_register(y) {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0x6 => {
                        chip8.write_register(x, kk);
                        chip8.pc += 2;
                        println!("Write into register {:02X} value {:02X}", x, kk);
                },
                0x7 => {
                        let data = chip8.read_register(x) + kk;
                        println!("Set V{:02X} = {} + {}.", x, chip8.read_register(x), kk);
                        chip8.write_register(x, data);
                        chip8.pc += 2;
                },
                0x8 => match chunk[1] & 0x0F {
                        0x0 => {
                                let y = chip8.read_register(y);
                                chip8.write_register(x, y);
                                println!("Set register {:02X} to {:02X}", x, y);
                                chip8.pc += 2;
                                },
                        0x1 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let data = x_value | y_value;
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                                println!("Set register {:02X} to {:08b} = {:08b} OR {:08b}.", x, data, x_value, y_value);
                        },
                        0x2 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let data = x_value & y_value;
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                                println!("Set register {:02X} to {:08b} = {:08b} AND {:08b}.", x, data, x_value, y_value);
                        },
                        0x3 => {
                                let x_value = chip8.read_register(x);
                                let y_value = chip8.read_register(y);
                                let data = x_value ^ y_value;
                                chip8.write_register(x, data);
                                chip8.pc += 2;
                                println!("Set register {:02X} to {:08b} = {:08b} XOR {:08b}.", x, data, x_value, y_value);
                        },
                        0x4 => {
                                let x_value = chip8.read_register(x) as u16;
                                let y_value = chip8.read_register(y) as u16;
                                let data = x_value + y_value;
                                println!("Set register {:02X} to value {:02X} = {:02X} + {:02X}", x, data, x_value, y_value);
                                chip8.write_register(x, data as u8);
                                if data > 0xFF {
                                        chip8.write_register(0xF, 1);
                                }
                                chip8.pc += 2;
                        },
                        0x5 => {
                                let x_value = chip8.read_register(x) as i8;
                                let y_value = chip8.read_register(y) as i8;
                                let data = x_value - y_value;
                                if data < 0 {
                                        chip8.write_register(0xF, 1);
                                }
                                println!("Set register {:02X} to value {:02X} = {:02X} - {:02X}", x, data, x_value, y_value);
                                chip8.write_register(x, data as u8);
                                chip8.pc += 2;
                        },
                        0x6 => {
                                let data = chip8.read_register(y); 
                                chip8.write_register(0xF, data & 0x00000001);
                                chip8.write_register(x, data << 1);
                                chip8.pc += 2
                        },
                        0x7 => panic!("unimplemented"),
                        0xE => panic!("unimplemented"),
                        _ => println!("0x8 ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                },
                0x9 => {
                        println!("Skip next instruction if register {:02X} with value {} != {}", x, chip8.read_register(x), chip8.read_register(y));
                        if chip8.read_register(x) != chip8.read_register(y) {
                                chip8.pc += 4;
                        } else {
                                chip8.pc += 2;
                        }
                },
                0xA => {
                        chip8.write_i(nnn);
                        chip8.pc += 2;
                        println!("Set I to {:02X}", nnn);
                        },
                0xB => {
                        println!("Jump to {} + V0{} = {}", nnn, chip8.read_register(0x0), nnn + chip8.read_register(x) as u16);
                        chip8.pc = nnn + chip8.read_register(0x0) as u16;
                },
                0xC => {
                        println!("generate random number to register {:02X}", x);
                        chip8.write_register(x, 12_u8);
                        chip8.pc += 2;
                },
                0xD => {
                                let i = chip8.read_i();
                                let x = chip8.read_register(x);
                                let mut y = chip8.read_register(y);
                                println!("Draw sprite at {:02X} at x: {} y: {}", i, y, x);
                                for address in i..i + n as u16 {
                                        let sprite = chip8.read_ram(address);
                                        println!("Sprite: {:08b}", sprite);
                                        chip8.write_sprite_to_window(&sprite, y, x);
                                        y += 1;
                                }
                                chip8.pc += 2;
                        },
                0xE => match chunk[1] {
                        0x9E => {
                                // IMPLEMENT KEYBOARD!!!!
                                println!("Skip next instruction if key in register {} with value of {} is pressed", x, chip8.read_register(x));
                                println!("{:?}", chip8);
                                let key = chip8.get_key();
                                println!("Key: pressed {}", key);
                                if key == chip8.read_register(x){
                                        chip8.pc += 4;
                                } else {
                                        chip8.pc += 2;
                                }
                        },
                        0xA1 => {
                                // IMPLEMENT KEYBOARD!!!!
                                println!("Keyboard function not implemented");
                                println!("Skip next instruction if key in register {} with value of {} not pressed", x, chip8.read_register(x));
                                println!("{:?}", chip8);
                                let key = chip8.get_key();
                                println!("Key: pressed {}", key);
                                if key != chip8.read_register(x){
                                        chip8.pc += 4;
                                } else {
                                        chip8.pc += 2;
                                }
                        },
                        _ => {
                                println!("0xE Error {:02X} {:02X}", chunk[0], chunk[1]);
                                panic!("Error");
                                },
                },
                0xF => match chunk[1] {
                        0x07 => {
                                println!("Writing delay timer {} to register {:02X}", chip8.get_delay_timer(), x);
                                chip8.write_register(x, chip8.get_delay_timer());
                                chip8.pc += 2;
                        },
                        0x0A => {
                                //IMPLEMENT KEYBOARD Blocking
                                println!("Blocking key");
                                while chip8.get_key() == 0xFF {
                                        chip8.pc += 0;
                                }
                                chip8.pc += 2;
                        },
                        0x15 => {
                                // set delay timer
                                chip8.set_delay_timer(chip8.read_register(x));
                                println!("Delay timer seet to: {}", chip8.read_register(x));
                                chip8.pc += 2;
                        },
                        0x18 => {
                                println!("Sound unimplemented");
                                chip8.pc += 2;
                        },
                        0x1E => {
                                let i = chip8.read_i();
                                let x = chip8.read_register(x) as u16;
                                println!("Set I to {}", i + x);
                                chip8.write_i(i + x);
                                chip8.pc += 2;
                        
                        },
                        0x29 => {
                                println!("Set I to font adress of {:02X}", x);
                                chip8.write_i(x as u16 * 5);
                                chip8.pc += 2;
                        },
                        0x33 => {       
                                        let digit = chip8.read_register(x);
                                        let h = (digit / 10_u8.pow(0)) % 10;
                                        let t = (digit / 10_u8.pow(1)) % 10;
                                        let s = (digit / 10_u8.pow(2)) % 10;
                                        println!("BCD: {} = {} {} {}", digit, h, t, s);
                                        chip8.write_ram(chip8.read_i(), h);
                                        chip8.write_ram(chip8.read_i() + 1, t);
                                        chip8.write_ram(chip8.read_i() + 2, s);
                                        chip8.pc += 2;
                        },
                        0x55 => panic!("unimplemented"),
                        0x65 => {       

                                        let start_address = chip8.read_i();
                                        println!("Write to registers 0 through {:02X} from memory starting at location {:02X}.", x, start_address);
                                        for i in 0..=x {
                                                let data = chip8.read_ram(start_address + i as u16);
                                                println!("Write to register {} with data {}", x, data);
                                                chip8.write_register(i, data);
                                        }
                                        // HMM?!
                                        //chip8.write_i(start_address + x as u16 + 1);
                                        chip8.pc += 2;
                        },
                        _ => {
                                println!("0xF ERROR {:02X} {:02X}", chunk[0], chunk[1]);
                                panic!("Error");
                        },
                },
                _ => println!("Error"),
            };
}