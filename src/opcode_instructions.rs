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
                                0xE => println!("unimplemented"),
                                0x0 => println!("unimplemented"),
                                _ => println!("ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                        },
                        _ => println!("unimplemented"),
                },
                0x2 => println!("unimplemented"),
                0x1 => println!("unimplemented"),
                0x3 => {
                                // Skip next instruction if Vx = kk.
                                if chip8.read_register(x) == kk {
                                        chip8.pc += 2;
                                } else {
                                        chip8.pc += 1;
                                }
                        },
                0x4 => println!("unimplemented"),
                0x5 => println!("unimplemented"),
                0x6 => {
                        chip8.write_register(x, kk);
                        //println!("{: <5} V{:02X} {:02X}", "LD", x, kk);
                        },
                0x7 => {
                        //Set Vx = Vx + kk.
                        let data = chip8.read_register(x) + kk;
                        chip8.write_register(x, data);
                        },
                0x8 => match chunk[1] & 0x0F {
                        0x0 => println!("unimplemented"),
                        0x1 => println!("unimplemented"),
                        0x2 => println!("unimplemented"),
                        0x3 => println!("unimplemented"),
                        0x4 => println!("unimplemented"),
                        0x5 => println!("unimplemented"),
                        0x6 => println!("unimplemented"),
                        0x7 => println!("unimplemented"),
                        0xE => println!("unimplemented"),
                        _ => println!("0x8 ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                },
                0x9 => println!("unimplemented"),
                0xA => {
                        chip8.write_i(nnn);
                        //println!("{: <5} I{:02X}", "LD", nnn);
                        },
                0xB => println!("unimplemented"),
                0xC => println!("unimplemented"),
                0xD => {
                                let i = chip8.read_i();
                                println!("Register I in draw function: {:02X}", i);
                                for address in i..=i + n as u16 {
                                        let sprite = chip8.read_ram(address);
                                        chip8.write_to_display(x, y, sprite);
                                }
                                //chip8.print_display_buffer();
                        },
                0xE => match chunk[1] {
                        0x9E => println!("unimplemented"),
                        0xA1 => println!("unimplemented"),
                        _ => println!("0xE Error {:02X} {:02X}", chunk[0], chunk[1]),
                },
                0xF => match chunk[1] {
                        0x07 => println!("unimplemented"),
                        0x0A => println!("unimplemented"),
                        0x15 => println!("unimplemented"),
                        0x18 => println!("unimplemented"),
                        0x1E => println!("unimplemented"),
                        0x29 => println!("unimplemented"),
                        0x33 => println!("unimplemented"),
                        0x55 => println!("unimplemented"),
                        0x65 => println!("unimplemented"),
                        _ => println!("0xF ERROR {:02X} {:02X}", chunk[0], chunk[1]),
                },
                _ => println!("Error"),
            };
}