use super::opcodes::Opcodes;

// todo!: verify the types
pub(crate) struct Instruction {
    pub(crate) opcode: Opcodes,
    pub(crate) rd: u32,
    pub(crate) funct3: u32,
    pub(crate) rs1: u32,
    pub(crate) rs2: u32,
    pub(crate) funct7: u32,
    pub(crate) imm: u32,
    pub(crate) imm_1: u32,
    pub(crate) succ: u32,
    pub(crate) pred: u32,
    pub(crate) fm: u32,
}

impl Instruction {
    fn new() -> Self {
        Self {
            opcode: Opcodes::Default,
            rd: 0,
            funct3: 0,
            rs1: 0,
            rs2: 0,
            funct7: 0,
            imm: 0,
            imm_1: 0,
            succ: 0,
            pred: 0,
            fm: 0,
        }
    }

    pub(crate) fn decode(instruction: &[u8]) -> Self {
        let instr = into_u32(instruction);
        let mut res = Instruction::new();

        if instr & 0x7F == 0x33 {
            // R type
            res.rd = (instr >> 7) & 0xF;
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.rs2 = (instr >> 20) & 0xF;
            res.funct7 = instr >> 25;

            match res.funct3 {
                0x0 => match res.funct7 {
                    0x00 => res.opcode = Opcodes::Add,
                    0x20 => res.opcode = Opcodes::Sub,
                    _ => panic!("Invalid instruction"),
                },
                0x1 => res.opcode = Opcodes::Sll,
                0x2 => res.opcode = Opcodes::Slt,
                0x3 => res.opcode = Opcodes::Sltu,
                0x4 => res.opcode = Opcodes::Xor,
                0x5 => match res.funct7 {
                    0x00 => res.opcode = Opcodes::Srl,
                    0x20 => res.opcode = Opcodes::Sra,
                    _ => panic!("Invalid instruction"),
                },
                0x6 => res.opcode = Opcodes::Or,
                0x7 => res.opcode = Opcodes::And,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0x13 {
            // I type
            res.rd = (instr >> 7) & 0xF;
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.imm = instr >> 20;

            match res.funct3 {
                0x00 => res.opcode = Opcodes::Addi,
                0x01 => {
                    res.funct7 = (instr >> 25) & 0x3F;
                    res.opcode = Opcodes::Slli;
                }
                0x02 => res.opcode = Opcodes::Slti,
                0x03 => res.opcode = Opcodes::Sltiu,
                0x04 => res.opcode = Opcodes::Xori,
                0x05 => {
                    res.funct7 = (instr >> 25) & 0x3F;
                    match res.funct7 {
                        0x00 => res.opcode = Opcodes::Srli,
                        0x20 => res.opcode = Opcodes::Srai,
                        _ => panic!("Invalid instruction"),
                    };
                }
                0x06 => res.opcode = Opcodes::Ori,
                0x07 => res.opcode = Opcodes::Andi,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0x3 {
            // Load I type
            res.rd = (instr >> 7) & 0xF;
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.imm = instr >> 20;

            match res.funct3 {
                0x00 => res.opcode = Opcodes::Lb,
                0x01 => res.opcode = Opcodes::Lh,
                0x02 => res.opcode = Opcodes::Lw,
                0x04 => res.opcode = Opcodes::Lbu,
                0x05 => res.opcode = Opcodes::Lhu,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0x23 {
            // S type
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.rs2 = (instr >> 20) & 0xF;
            res.imm = (instr >> 7) & 0xF;
            res.imm_1 = instr >> 25;

            match res.funct3 {
                0x0 => res.opcode = Opcodes::Sb,
                0x1 => res.opcode = Opcodes::Sh,
                0x2 => res.opcode = Opcodes::Sw,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0x63 {
            // B type
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.rs2 = (instr >> 20) & 0xF;
            res.imm = (instr >> 7) & 0xF;
            res.imm_1 = instr >> 25;

            match res.funct3 {
                0x0 => res.opcode = Opcodes::Beq,
                0x1 => res.opcode = Opcodes::Bne,
                0x4 => res.opcode = Opcodes::Blt,
                0x5 => res.opcode = Opcodes::Bge,
                0x6 => res.opcode = Opcodes::Bltu,
                0x7 => res.opcode = Opcodes::Bgeu,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0x6f {
            // J type
            res.opcode = Opcodes::Jal;
            res.rd = (instr >> 7) & 0xF;
            res.imm = instr >> 12;
            res
        } else if instr & 0x7F == 0x67 {
            // I type
            res.rd = (instr >> 7) & 0xF;
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.imm = instr >> 20;
            res.opcode = Opcodes::Jalr;

            res
        } else if instr & 0x7F == 0x37 {
            // U type

            res.rd = (instr >> 7) & 0xF;
            res.imm = instr >> 12;
            res.opcode = Opcodes::Lui;
            res
        } else if instr & 0x7F == 0x17 {
            // U type
            res.rd = (instr >> 7) & 0xF;
            res.imm = instr >> 12;
            res.opcode = Opcodes::Auipc;
            res
        } else if instr & 0x7F == 0x73 {
            // I type

            res.rd = (instr >> 7) & 0xF;
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;

            match res.imm {
                0x0 => res.opcode = Opcodes::Ecall,
                0x1 => res.opcode = Opcodes::Ebreak,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0xF {
            // Fence

            res.opcode = Opcodes::Fence;
            res.rd = (instr >> 7) & 0xF;
            res.funct3 = (instr >> 12) & 0x3;
            res.rs1 = (instr >> 15) & 0xF;
            res.succ = (instr >> 20) & 0x7;
            res.pred = (instr >> 24) & 0x7;
            res.fm = instr >> 28;

            res
        } else {
            panic!("Invalid instruction")
        }
    }
}

// u32 returned is le(little endian)
pub(crate) fn into_u32(instr: &[u8]) -> u32 {
    (instr[0] as u32) << 24 | (instr[1] as u32) << 16 | (instr[2] as u32) << 8 | (instr[3] as u32)
}

// converts u32 to byte
pub(crate) fn into_byte(val: u32) -> Vec<u8> {
    vec![
        (val >> 24) as u8,
        ((val >> 16) & 0xFF) as u8,
        ((val >> 8) & 0xFF) as u8,
        (val & 0xFF) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use crate::vm::{
        instruction::{into_byte, into_u32, Instruction},
        opcodes::Opcodes,
    };

    #[test]
    fn test_decode_add_instruction() {
        // Assembly instruction
        // https://riscv-programming.org/book/riscv-book.html#pfd - pg 43
        //
        //               rs2   rs1    funct3  rd     opcode
        // 0x00c58533 -> 1100  01011  000     01010  0110011 = add x10, x11, x12
        let instr_as_bytes: [u8; 4] = [0x00, 0xc5, 0x85, 0x33];
        let val = into_u32(&instr_as_bytes);

        dbg!(format!("instr : {:b}", val));

        dbg!(format!("Opcode : {:b}", val & 0x7F));
        dbg!(format!("rd : {:b}", (val >> 7) & 0xF));
        dbg!(format!("funct3 : {:b}", (val >> 12) & 0x3));
        dbg!(format!("rs1 : {:b}", (val >> 15) & 0xF));
        dbg!(format!("rs2 : {:b}", (val >> 20) & 0xF));
        dbg!(format!("funct7 : {:b}", val >> 25));

        let instr = Instruction::decode(&instr_as_bytes);
        assert_eq!(instr.opcode, Opcodes::Add);
        assert_eq!(instr.rd, 10);
        assert_eq!(instr.rs1, 11);
        assert_eq!(instr.rs2, 12);

        let res = into_byte(val);
        assert_eq!(res, instr_as_bytes);
    }
}
