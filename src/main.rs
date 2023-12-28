use std::env;
use std::fs;
use std::fs::File;
use std::str;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
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
        let opcode = Opcode::from_binary((instruction >> 8) as u8);
        let direction = Direction::from_u16((instruction >> 9) & 1);
        let wide = (instruction >> 8) & 1 != 0;
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

    //fn from_string(&str) -> Instruction {
        // mov cx, bx
        // first word is opcode
        // second word -, is reg1
        // third word is reg2
        // D?
        // W?
        // MOD?
    //}

    fn to_string(&self) -> String {
        let reg1 = if self.direction == Direction::ToReg1 {
            register_to_string(self.register1, self.wide)
        } else {
            register_to_string(self.register2, self.wide)
        };

        let reg2 = if self.direction == Direction::ToReg1 {
            register_to_string(self.register2, self.wide)
        } else { 
            register_to_string(self.register1, self.wide)
        };

        format!("{} {}, {}", self.opcode.to_string(), reg1, reg2)
    }
}

#[derive(Debug)]
enum Opcode {
    MOV
}

impl Opcode {
    fn from_string(name: &str) -> Opcode {
        match name {
            "mov" => Opcode::MOV,
            _ => panic!("Unknown Opcode: {}", name)
        }
    }

    fn from_binary(binary: u8) -> Opcode {
        match binary >> 2 { // Bit shift to remove the D and W bits
            0b100010 => Opcode::MOV,
            _ => panic!("Unknown Opcode: {}", binary >> 2)
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Opcode::MOV => "mov",
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    ToReg1,
    FromReg1
}

impl Direction {
    fn from_u16(value: u16) -> Direction {
        match value {
            0 => Direction::ToReg1,
            1 => Direction::FromReg1,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

#[derive(Debug)]
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
        "ass" => {
            // read assembly file
            let reader = BufReader::new(File::open(filename).expect("Unable to open file"));
            // parse each line as an instruction
            for line in reader.lines() {
                //Instruction::from_string();
            }
        },
        "dis" => {
            let bytes = fs::read(path).expect("Unable to read file");
            let instructions = bytes_to_instructions(bytes.as_slice());
            for instruction in instructions {
                println!("{}", instruction.to_string());
            }
        },
        "binarify" => {
            let file = fs::read_to_string(path.clone());
            let binary = utf8_to_bytes(file.unwrap());
            fs::write(path, binary).expect("Unable to write to file");
        },
        _ => panic!("{} is not a valid argument", args[0])
    };
}

fn print_file_binary(path: &str) {
    let contents = fs::read(path).expect("Unable to read file");
    for c in contents {
        println!("{:b}", c);
    }
}

fn bytes_to_instructions(binary: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut i = 0;
    while i < binary.len() / 2 {
        let instruction = Instruction::from_u16(((binary[i * 2] as u16) << 8) + binary[i * 2 + 1] as u16);
        instructions.push(instruction);

        i += 1;
    }

    instructions
}

fn hex_to_string(hex: Vec<u8>) -> String {
    let mut string = "".to_string();
    for c in hex {
        string.push(c as char);
    }
    string.to_string()
}

fn utf8_to_bytes(string: String) -> Vec<u8> {
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

fn register_to_string(register: u8, wide: bool) -> String {
    match register {
        0b000 => {
            match wide {
                true => "ax".to_string(),
                false => "al".to_string()
            }
        },
        0b001 => {
            match wide {
                true => "cx".to_string(),
                false => "cl".to_string()
            }
        },
        0b010 => {
            match wide {
                true => "dx".to_string(),
                false => "dl".to_string()
            }
        },
        0b011 => {
            match wide {
                true => "bx".to_string(),
                false => "bl".to_string()
            }
        },
        0b100 => {
            match wide {
                true => "sp".to_string(),
                false => "ah".to_string()
            }
        },
        0b101 => {
            match wide {
                true => "bp".to_string(),
                false => "ch".to_string()
            }
        },
        0b110 => {
            match wide {
                true => "si".to_string(),
                false => "dh".to_string()
            }
        },
        0b111 => {
            match wide {
                true => "di".to_string(),
                false => "bh".to_string()
            }
        },
        _ => panic!("Invalid register: {}", register)
    }
}
