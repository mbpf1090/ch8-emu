use super::Chip8;

pub fn run_opcode(chunk: &[u8], chip8: &mut Chip8::Chip8){

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
                                0xE => panic!("unimplemented"),
                                0x0 => panic!("unimplemented"),
                                _ => println!("ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                        },
                        _ => panic!("unimplemented"),
                },
                0x2 => {
                                chip8.stack.push_back(chip8.pc);
                                chip8.pc = nnn;
                        },
                0x1 => {
                                chip8.pc = nnn;
                                println!("Jump to {:0X}", nnn)
                        },
                0x3 => {
                                // Skip next instruction if Vx = kk.
                                if chip8.read_register(x) == kk {
                                        chip8.pc += 4;
                                } else {
                                        chip8.pc += 2;
                                }
                        },
                0x4 => panic!("unimplemented"),
                0x5 => panic!("unimplemented"),
                0x6 => {
                        chip8.write_register(x, kk);
                        chip8.pc += 2;
                        //println!("{: <5} V{:02X} {:02X}", "LD", x, kk);
                        },
                0x7 => {
                        //Set Vx = Vx + kk.
                        let data = chip8.read_register(x) + kk;
                        chip8.write_register(x, data);
                        chip8.pc += 2;
                        },
                0x8 => match chunk[1] & 0x0F {
                        0x0 => panic!("unimplemented"),
                        0x1 => panic!("unimplemented"),
                        0x2 => panic!("unimplemented"),
                        0x3 => panic!("unimplemented"),
                        0x4 => panic!("unimplemented"),
                        0x5 => panic!("unimplemented"),
                        0x6 => panic!("unimplemented"),
                        0x7 => panic!("unimplemented"),
                        0xE => panic!("unimplemented"),
                        _ => println!("0x8 ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                },
                0x9 => panic!("unimplemented"),
                0xA => {
                        chip8.write_i(nnn);
                        chip8.pc += 2;
                        //println!("{: <5} I{:02X}", "LD", nnn);
                        },
                0xB => panic!("unimplemented"),
                0xC => panic!("unimplemented"),
                0xD => {
                                let i = chip8.read_i();
                                println!("Register I in draw function: {:02X}", i);
                                //for address in i..=i + n as u16 {
                                //        let sprite = chip8.read_ram(address);
                                //        chip8.write_to_display(x, y, sprite);
                                //}
                                //chip8.print_display_buffer();
                                chip8.pc += 2;
                        },
                0xE => match chunk[1] {
                        0x9E => panic!("unimplemented"),
                        0xA1 => panic!("unimplemented"),
                        _ => println!("0xE Error {:02X} {:02X}", chunk[0], chunk[1]),
                },
                0xF => match chunk[1] {
                        0x07 => panic!("unimplemented"),
                        0x0A => panic!("unimplemented"),
                        0x15 => panic!("unimplemented"),
                        0x18 => panic!("unimplemented"),
                        0x1E => panic!("unimplemented"),
                        0x29 => panic!("unimplemented"),
                        0x33 => {
                                        let digit = chip8.read_register(x);
                                        let h = (digit / 10_u8.pow(0)) % 10;
                                        let t = (digit / 10_u8.pow(1)) % 10;
                                        let s = (digit / 10_u8.pow(2)) % 10;
                                        chip8.write_ram(chip8.read_i(), h);
                                        chip8.write_ram(chip8.read_i() + 1, t);
                                        chip8.write_ram(chip8.read_i() + 2, s);
                                        chip8.pc += 2;
                                },
                        0x55 => panic!("unimplemented"),
                        0x65 => panic!("unimplemented"),
                        _ => println!("0xF ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                },
                _ => println!("Error"),
            };
}