use std::fmt::Display;

use super::{
    opcodes::{OpcodeTypes, Opcodes},
    sign_extend, Registers,
};

// todo!: verify the types
#[derive(Debug)]
pub(crate) struct Instruction {
    pub(crate) opcode: Opcodes,
    pub(crate) rd: Registers,
    pub(crate) funct3: u32,
    pub(crate) rs1: Registers,
    pub(crate) rs2: Registers,
    pub(crate) funct7: u32,
    pub(crate) imm: u32,
    pub(crate) succ: u32,
    pub(crate) pred: u32,
    pub(crate) fm: u32,
}

impl Instruction {
    fn new() -> Self {
        Self {
            opcode: Opcodes::Default,
            rd: Registers::Default,
            funct3: 0,
            rs1: Registers::Default,
            rs2: Registers::Default,
            funct7: 0,
            imm: 0,
            succ: 0,
            pred: 0,
            fm: 0,
        }
    }

    pub(crate) fn decode(instruction: &[u8]) -> Self {
        let instr = u32::from_le_bytes(instruction.try_into().unwrap());
        let mut res = Instruction::new();

        if instr & 0x7F == 0x33 {
            // R type
            res.rd = ((instr >> 7) & 0x1F).into();
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.rs2 = ((instr >> 20) & 0x1F).into();
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
            // I type (immediate)
            res.rd = ((instr >> 7) & 0x1F).into();
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.imm = sign_extend(instr >> 20, 11);

            match res.funct3 {
                0x00 => res.opcode = Opcodes::Addi,
                0x01 => {
                    // res.funct7 = (instr >> 25) & 0x3F;
                    res.funct7 = (res.imm >> 5) & 0x7F;
                    res.opcode = Opcodes::Slli;
                }
                0x02 => res.opcode = Opcodes::Slti,
                0x03 => res.opcode = Opcodes::Sltiu,
                0x04 => res.opcode = Opcodes::Xori,
                0x05 => {
                    // res.funct7 = (instr >> 25) & 0x3F;
                    res.funct7 = (res.imm >> 5) & 0x7F;
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
            // I type(load)
            res.rd = ((instr >> 7) & 0x1F).into();
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.imm = sign_extend(instr >> 20, 11);

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
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.rs2 = ((instr >> 20) & 0x1F).into();
            res.imm = sign_extend((instr >> 25) << 5 | ((instr >> 7) & 0x1F), 11);

            match res.funct3 {
                0x0 => res.opcode = Opcodes::Sb,
                0x1 => res.opcode = Opcodes::Sh,
                0x2 => res.opcode = Opcodes::Sw,
                _ => panic!("Invalid instruction"),
            }

            res
        } else if instr & 0x7F == 0x63 {
            // B type
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.rs2 = ((instr >> 20) & 0x1F).into();
            res.imm = sign_extend(
                (instr >> 31) << 12
                    | ((instr >> 7) & 1) << 11
                    | ((instr >> 25) & 0x3F) << 5
                    | ((instr >> 8) & 0xF) << 1,
                12,
            );

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
            res.rd = ((instr >> 7) & 0x1F).into();
            res.imm = sign_extend(
                (instr >> 31) << 20
                    | ((instr >> 12) & 0xFF) << 19
                    | ((instr >> 20) & 0x1) << 11
                    | ((instr >> 21) & 0x7FF) << 1,
                20,
            );
            res
        } else if instr & 0x7F == 0x67 {
            // I type (jalr)
            res.rd = ((instr >> 7) & 0x1F).into();
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.imm = sign_extend(instr >> 20, 11);
            res.opcode = Opcodes::Jalr;

            res
        } else if instr & 0x7F == 0x37 {
            // U type (lui)

            res.rd = ((instr >> 7) & 0x1F).into();
            res.imm = instr >> 12;
            res.opcode = Opcodes::Lui;
            res
        } else if instr & 0x7F == 0x17 {
            // U type (auipc)
            res.rd = ((instr >> 7) & 0x1F).into();
            res.imm = instr >> 12;
            res.opcode = Opcodes::Auipc;
            res
        } else if instr & 0x7F == 0x73 {
            // I type (Ecalls)

            res.rd = ((instr >> 7) & 0x1F).into();
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            // res.imm = sign_extend(instr >> 20, 11);
            res.imm = instr >> 20;

            match res.imm {
                0x0 => res.opcode = Opcodes::Ecall,
                0x1 => res.opcode = Opcodes::Ebreak,
                _ => res.opcode = Opcodes::EAny,
            }

            res
        } else if instr & 0x7F == 0xF {
            // Fence

            res.opcode = Opcodes::Fence;
            res.rd = ((instr >> 7) & 0x1F).into();
            res.funct3 = (instr >> 12) & 0x7;
            res.rs1 = ((instr >> 15) & 0x1F).into();
            res.succ = (instr >> 20) & 0xF;
            res.pred = (instr >> 24) & 0xF;
            res.fm = instr >> 28;

            res
        } else {
            dbg!(instr);
            panic!("Invalid instruction")
        }
    }

    fn encode_to_instruction(instr: String) -> Self {
        let token: Vec<&str> = instr.split(" ").collect();
        let mut res = Instruction::new();
        let opcode = token[0];

        if opcode == "add" {
            res.opcode = Opcodes::Add;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x00;
            res.funct7 = 0x00;
            res
        } else if opcode == "sub" {
            res.opcode = Opcodes::Sub;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x00;
            res.funct7 = 0x20;
            res
        } else if opcode == "xor" {
            res.opcode = Opcodes::Xor;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x4;
            res.funct7 = 0x00;
            res
        } else if opcode == "or" {
            res.opcode = Opcodes::Or;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x6;
            res.funct7 = 0x00;
            res
        } else if opcode == "and" {
            res.opcode = Opcodes::And;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x7;
            res.funct7 = 0x00;
            res
        } else if opcode == "sll" {
            res.opcode = Opcodes::Sll;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x1;
            res.funct7 = 0x00;
            res
        } else if opcode == "srl" {
            res.opcode = Opcodes::Srl;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x5;
            res.funct7 = 0x00;
            res
        } else if opcode == "sra" {
            res.opcode = Opcodes::Sra;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x5;
            res.funct7 = 0x20;
            res
        } else if opcode == "slt" {
            res.opcode = Opcodes::Slt;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x2;
            res.funct7 = 0x00;
            res
        } else if opcode == "sltu" {
            res.opcode = Opcodes::Sltu;
            res.rd = token[1].into();
            res.rs1 = token[2].into();
            res.rs2 = token[3].into();
            res.funct3 = 0x3;
            res.funct7 = 0x00;
            res
        } else if opcode == "addi" {
            todo!()
        } else if opcode == "xori" {
            todo!()
        } else if opcode == "ori" {
            todo!()
        } else if opcode == "andi" {
            todo!()
        } else if opcode == "slli" {
            todo!()
        } else if opcode == "srli" {
            todo!()
        } else if opcode == "srai" {
            todo!()
        } else if opcode == "slti" {
            todo!()
        } else if opcode == "sltiu" {
            todo!()
        } else if opcode == "lb" {
            res.opcode = Opcodes::Lb;
            res
        } else if opcode == "lh" {
            todo!()
        } else if opcode == "lw" {
            todo!()
        } else if opcode == "lbu" {
            todo!()
        } else if opcode == "lhu" {
            todo!()
        } else if opcode == "sb" {
            todo!()
        } else if opcode == "sh" {
            todo!()
        } else if opcode == "sw" {
            todo!()
        } else if opcode == "beq" {
            todo!()
        } else if opcode == "bne" {
            todo!()
        } else if opcode == "blt" {
            todo!()
        } else if opcode == "bge" {
            todo!()
        } else if opcode == "bltu" {
            todo!()
        } else if opcode == "bgeu" {
            todo!()
        } else if opcode == "jal" {
            todo!()
        } else if opcode == "jalr" {
            todo!()
        } else if opcode == "lui" {
            todo!()
        } else if opcode == "auipc" {
            todo!()
        } else if opcode == "ecall" {
            todo!()
        } else if opcode == "ebreak" {
            todo!()
        } else if opcode == "fence" {
            todo!()
        } else if opcode == "eany" {
            todo!()
        } else {
            panic!("Invalid opcode")
        }
    }

    fn encode(&self) -> u32 {
        match self.opcode {
            Opcodes::Add => todo!(),
            Opcodes::Sub => todo!(),
            Opcodes::Xor => todo!(),
            Opcodes::Or => todo!(),
            Opcodes::And => todo!(),
            Opcodes::Sll => todo!(),
            Opcodes::Srl => todo!(),
            Opcodes::Sra => todo!(),
            Opcodes::Slt => todo!(),
            Opcodes::Sltu => todo!(),
            Opcodes::Addi => todo!(),
            Opcodes::Xori => todo!(),
            Opcodes::Ori => todo!(),
            Opcodes::Andi => todo!(),
            Opcodes::Slli => todo!(),
            Opcodes::Srli => todo!(),
            Opcodes::Srai => todo!(),
            Opcodes::Slti => todo!(),
            Opcodes::Sltiu => todo!(),
            Opcodes::Lb => todo!(),
            Opcodes::Lh => todo!(),
            Opcodes::Lw => todo!(),
            Opcodes::Lbu => todo!(),
            Opcodes::Lhu => todo!(),
            Opcodes::Sb => todo!(),
            Opcodes::Sh => todo!(),
            Opcodes::Sw => todo!(),
            Opcodes::Beq => todo!(),
            Opcodes::Bne => todo!(),
            Opcodes::Blt => todo!(),
            Opcodes::Bge => todo!(),
            Opcodes::Bltu => todo!(),
            Opcodes::Bgeu => todo!(),
            Opcodes::Jal => todo!(),
            Opcodes::Jalr => todo!(),
            Opcodes::Lui => todo!(),
            Opcodes::Auipc => todo!(),
            Opcodes::Ecall => todo!(),
            Opcodes::Ebreak => todo!(),
            Opcodes::Fence => todo!(),
            //
            Opcodes::Default => 0,
            Opcodes::EAny => 0,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode {
            Opcodes::Add => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Sub => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Xor => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Or => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::And => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Sll => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Srl => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Sra => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Slt => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Sltu => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.rs2
            )),
            Opcodes::Addi => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Xori => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Ori => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Andi => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Slli => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Srli => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Srai => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Slti => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Sltiu => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Lb => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Lh => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Lw => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Lbu => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Lhu => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Sb => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rs1, self.rs2, self.imm as i32
            )),
            Opcodes::Sh => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rs1, self.rs2, self.imm as i32
            )),
            Opcodes::Sw => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rs1, self.rs2, self.imm as i32
            )),
            Opcodes::Beq => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rs1, self.rs2)),
            Opcodes::Bne => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rs1, self.rs2)),
            Opcodes::Blt => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rs1, self.rs2)),
            Opcodes::Bge => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rs1, self.rs2)),
            Opcodes::Bltu => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rs1, self.rs2)),
            Opcodes::Bgeu => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rs1, self.rs2)),
            Opcodes::Jal => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.rd, self.imm as i32
            )),
            Opcodes::Jalr => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.rd, self.rs1, self.imm as i32
            )),
            Opcodes::Lui => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rd, self.imm)),
            Opcodes::Auipc => f.write_fmt(format_args!("{} {} {}", self.opcode, self.rd, self.imm)),
            Opcodes::Ecall => f.write_fmt(format_args!("{} {}", self.opcode, self.imm)),
            Opcodes::Ebreak => f.write_fmt(format_args!("{} {}", self.opcode, self.imm)),
            Opcodes::Fence => f.write_fmt(format_args!("{}", self.opcode)),
            //
            Opcodes::Default => f.write_fmt(format_args!("{}", self.opcode)),
            Opcodes::EAny => f.write_fmt(format_args!("{} {}", self.opcode, self.imm)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{instruction::Instruction, opcodes::Opcodes};

    #[test]
    fn test_decode_add_instruction() {
        // Assembly instruction
        // https://riscv-programming.org/book/riscv-book.html#pfd - pg 43
        //
        //               rs2   rs1    funct3  rd     opcode
        // 0x00c58533 -> 1100  01011  000     01010  0110011 = add x10, x11, x12
        let instr_as_bytes: [u8; 4] = [0x33, 0x85, 0xc5, 0x00];
        let val = u32::from_le_bytes(instr_as_bytes);

        let instr = Instruction::decode(&instr_as_bytes);
        assert_eq!(instr.opcode, Opcodes::Add);
        assert_eq!(instr.rd as u32, 10);
        assert_eq!(instr.rs1 as u32, 11);
        assert_eq!(instr.rs2 as u32, 12);
        assert_eq!(instr.funct3, 0);
        assert_eq!(instr.funct7, 0);

        let res = val.to_le_bytes();
        assert_eq!(res, instr_as_bytes);
    }

    #[test]
    fn test_endianess() {
        let val = [0x6f, 0x00, 0x00, 0x05];
        dbg!(format_args!("{:?}", Instruction::decode(&val)));
    }

    #[test]
    fn test_decode_instruction_i_type() {
        let val = 0x00C58513_u32.to_le_bytes();
        let instr = Instruction::decode(&val);
        assert_eq!(instr.imm, 12);
    }

    #[test]
    fn test_decode_instruction_s_type() {
        let val = 0x00822323_u32.to_le_bytes();
        let instr = Instruction::decode(&val);
        assert_eq!(instr.imm, 6);
    }

    #[test]
    fn test_decode_instruction_s_type_neg() {
        let val = 0xfe822d23_u32.to_le_bytes();
        let instr = Instruction::decode(&val);
        assert_eq!(instr.imm, -6_i32 as u32);
    }

    #[test]
    fn test_decode_instruction_beq_type() {
        let val = 0x00628a63_u32.to_le_bytes();
        dbg!(format_args!("{:b}", 0x00628a63));
        let instr = Instruction::decode(&val);
        dbg!(&instr);
        assert_eq!(instr.imm, 20);
    }

    #[test]
    fn test_decode_instruction_u_type() {
        let val = 0x000a42b7_u32.to_le_bytes();
        dbg!(format_args!("{:b}", 0x000a42b7));
        let instr = Instruction::decode(&val);
        dbg!(&instr);
        assert_eq!(instr.imm, 164);
    }

    #[test]
    fn test_decode_instruction_jal_type() {
        let val = 0x02c002ef_u32.to_le_bytes();
        dbg!(format_args!("{:b}", 0x02c002ef));
        let instr = Instruction::decode(&val);
        dbg!(&instr);
        assert_eq!(instr.imm, 44);
    }

    #[test]
    fn test_display_instruction() {
        let inst = 0xfe822d23_u32.to_le_bytes();
        let instr = Instruction::decode(&inst);
        println!("{}", instr);
    }
}
