struct Instruction {
    opcode: Opcode,
    direction: Direction,
    wide: bool,
    mode: Mode,
    register1: u8,
    register2: u8
}

enum Opcode {
    MOV
}

enum Direction {
    ToRegister,
    FromRegister
}

enum Mode {
    Register,
    Memory
}

fn main() {
}

fn decode_instruction(instruction: u16) -> Instruction {
}
