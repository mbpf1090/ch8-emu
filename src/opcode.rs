pub fn get_opcode(chunk: &[u8]) -> String {

    if chunk[0] == 0 && chunk[1] == 0 {
                let opcode = String::from("EMPTY");
                return opcode;
            }
            let nnn = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
            let n = chunk[1] & 0x0F;
            let x = chunk[0] & 0x0F;
            let y = chunk[1] >> 4;
            let kk = chunk[1];

            let opcode = match chunk[0] >> 4 {
                0x0 => match chunk[0] >> 3 {
                        0x0 => match chunk[1] & 0x0F {
                                0xE => String::from("RET"),
                                0x0 => String::from("CLS"),
                                _ => format!("ERROR {:02X} {:02X}", chunk[0], chunk[1])
                        },
                        _ => format!("{: <5} {:02X}", "SYS", nnn)
                },
                0x2 => format!("{: <5} {:2X}", "CALL", nnn),
                0x1 => format!("{: <5} {:<10X}", "JP", nnn),
                0x3 => format!("{: <5} {:02X} {:02X}", "SE", x, kk),
                0x4 => format!("{: <5} {:02X} {:02X}", "SNE", x, kk),
                0x5 => format!("{: <5} {:02X} {:02X}", "SE", x, y),
                0x6 => format!("{: <5} {:02X} {:02X}", "LD", x, kk),
                0x7 => format!("{: <5} {:02X} {:02X}", "ADD", x, kk),
                0x8 => match chunk[1] & 0x0F {
                        0x0 => format!("{: <5} {:02X} {:02X}", "LD", x, y),
                        0x1 => format!("{: <5} {:02X} {:02X}", "OR", x, y),
                        0x2 => format!("{: <5} {:02X} {:02X}", "AND", x, y),
                        0x3 => format!("{: <5} {:02X} {:02X}", "XOR", x, y),
                        0x4 => format!("{: <5} {:02X} {:02X}", "ADD", x, y),
                        0x5 => format!("{: <5} {:02X} {:02X}", "SUB", x, y),
                        0x6 => format!("{: <5} {:02X} {:02X}", "SHR", x, y),
                        0x7 => format!("{: <5} {:02X} {:02X}", "SUBN", x, y),
                        0xE => format!("{: <5} {:02X} {:02X}", "SHL", x, y),
                        _ => format!("0x8 ERROR {:02X} {:02X}", chunk[0], chunk[1])
                },
                0x9 => format!("{: <5} {:02X} {:02X}", "SNE", x, y),
                0xA => format!("{: <5} {:02X}", "LD I", nnn),
                0xB => format!("{: <5} {:02X}", "JP V0", nnn),
                0xC => format!("{: <5} {:02X} {:02X}", "RND", x, kk),
                0xD => format!("{: <5} {:02X} {:02X} {:02X}", "DRW", x, y, n),
                0xE => match chunk[1] {
                        0x9E => format!("{: <5} {:02X}", "SKP", x),
                        0xA1 => format!("{: <5} {:02X}", "SKNP", x,),
                        _ => format!("0xE Error {:02X} {:02X}", chunk[0], chunk[1])
                },
                0xF => match chunk[1] {
                        0x07 => format!("{: <5} {:02X}", "LD DT", x),
                        0x0A => format!("{: <5} {:02X}", "LD K", x),
                        0x15 => format!("{: <5} {:02X}", "LD DT", x),
                        0x18 => format!("{: <5} {:02X}", "LD ST", x),
                        0x1E => format!("{: <5} {:02X}", "ADD I", x),
                        0x29 => format!("{: <5} {:02X}", "LD F", x),
                        0x33 => format!("{: <5} {:02X}", "LD B", x),
                        0x55 => format!("{: <5} {:02X}", "LF I", x),
                        0x65 => format!("{: <5} {:02X}", "LD I", x),
                        _ => format!("0xF ERROR {:02X} {:02X}", chunk[0], chunk[1])
                },
                _ => String::from("none"),
            };
            opcode
}