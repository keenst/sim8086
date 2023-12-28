use std::env;
use std::fs;
use std::str;

struct Instruction {
    opcode: Opcode,
    direction: Direction,
    wide: bool,
    mode: Mode, // TODO: Implement displacement for MOD
    register1: u8,
    register2: u8
}

impl Instruction {
    fn from_u16(instruction: u16) -> Instruction {
        // 543210 9 8 76  543 210
        // OPCODE D W MOD R1  R2
        let opcode = binary_to_opcode((instruction >> 8) as u8);
        let direction = Direction::from_u16((instruction >> 9) & 1);
        let wide = (instruction >> 10) & 1 != 0;
        let mode = Mode::from_u16((instruction >> 6) & 0b11);
        let register1 = ((instruction >> 3) & 0b111) as u8;
        let register2 = (instruction & 0b111) as u8;

        Instruction {
            opcode,
            direction,
            wide,
            mode,
            register1,
            register2
        }
    }
}

enum Opcode {
    MOV
}

enum Direction {
    ToRegister,
    FromRegister
}

enum Action {
    Assemble,
    Disassemble,
    Binarify
}

impl Direction {
    fn from_u16(value: u16) -> Direction {
        match value {
            0 => Direction::ToRegister,
            1 => Direction::FromRegister,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

enum Mode {
    Memory,
    Register
}

impl Mode {
    fn from_u16(value: u16) -> Mode {
        if value == 0b11 { // R/M = 11
            Mode::Register
        } else {
            Mode::Memory
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Invalid amount of arguments");
    }


    let filename = args[2].as_str();

    let path_t = env::current_dir().unwrap();
    let path = format!("{}/{}", path_t.to_str().unwrap(), filename);

    match args[1].as_str() {
        "ass" => {},
        "dis" => {
            read_file(path.as_str());
        },
        "binarify" => {
            let file = fs::read_to_string(path.clone());
            let binary = string_to_binary(file.unwrap());
            fs::write(path, binary).expect("Unable to write to file");
        },
        _ => panic!("{} is not a valid argument", args[0])
    };

}

fn read_file(path: &str) {
    let contents = fs::read(path).expect("Unable to read file");
    for c in contents {
        println!("{:b}", c);
    }
}

fn binary_to_instructions(binary: Vec<u8>) -> Vec<Instruction> {

}

fn string_to_binary(string: String) -> Vec<u8> {
    let mut new_byte: Vec<bool> = Vec::new();
    let mut new_contents: Vec<u8> = Vec::new();

    for c in string.as_bytes() {
        match c {
            48 => new_byte.push(false),
            49 => new_byte.push(true),
            _ => continue
        }

        if new_byte.len() == 8 {
            let mut new_char: u8 = 0;
            let mut i = 0;
            while i < 8 {
                if new_byte[i] {
                    new_char += 1 << (7 - i) as u8;
                } else {
                    i += 1;
                    continue
                }

                i += 1;
            }

            new_contents.push(new_char);
            new_byte.clear();
        }
    }

    new_contents
}

fn binary_to_string(binary: Vec<u8>) -> String {
    match str::from_utf8(binary.as_slice()) {
        Ok(value) => value.to_string(),
        Err(error) => panic!("Invalid UTF-8 sequence: {}", error)
    }
}

fn binary_to_opcode(binary: u8) -> Opcode {
    match binary >> 2 { // Bit shift to remove the D and W bits
        0b100010 => Opcode::MOV,
        _ => panic!("Unknown Opcode: {}", binary >> 2)
    }
}
