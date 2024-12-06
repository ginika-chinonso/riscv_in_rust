use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub(crate) enum Opcodes {
    // Base interger instructions

    // Register (R)
    Add,
    Sub,
    Xor,
    Or,
    And,
    Sll,  // shift left logical
    Srl,  // shift right logical
    Sra,  // shift right arith
    Slt,  // set less than
    Sltu, // set less than (U)

    // Immediate (I)
    Addi,  // add immediate
    Xori,  // xor immediate
    Ori,   // or immediate
    Andi,  // and immediate
    Slli,  // shift left logical immediate
    Srli,  // shift right logical immediate
    Srai,  // shift right arith immediate
    Slti,  // set less than immediate
    Sltiu, // set less than immediate (U)

    // load
    Lb,  // load byte
    Lh,  // load half
    Lw,  // load word
    Lbu, // load byte (U)
    Lhu, // load half (U)

    // Store (S)
    Sb, // store byte
    Sh, // store half
    Sw, // store word

    // Branch (B)
    Beq,  // branch ==
    Bne,  // branch !=
    Blt,  // branch <
    Bge,  // branch >=
    Bltu, // branch < (U)
    Bgeu, // branch >= (U)

    Jal,  // jump and link(J)
    Jalr, // jump and link reg(I)

    Lui,    // load upper immediate(U)
    Auipc,  // add upper immediate to pc(U)
    Ecall,  // environment call(I)
    Ebreak, // environment break(I)
    Fence,  // memory fence(FENCE)

    // Default
    Default,
}

impl Display for Opcodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcodes::Add => f.write_str("ADD"),
            Opcodes::Sub => f.write_str("SUB"),
            Opcodes::Xor => f.write_str("XOR"),
            Opcodes::Or => f.write_str("OR"),
            Opcodes::And => f.write_str("AND"),
            Opcodes::Sll => f.write_str("SLL"),
            Opcodes::Srl => f.write_str("SRL"),
            Opcodes::Sra => f.write_str("SRA"),
            Opcodes::Slt => f.write_str("SLT"),
            Opcodes::Sltu => f.write_str("SLTU"),
            Opcodes::Addi => f.write_str("ADDI"),
            Opcodes::Xori => f.write_str("XORI"),
            Opcodes::Ori => f.write_str("ORI"),
            Opcodes::Andi => f.write_str("ANDI"),
            Opcodes::Slli => f.write_str("SLLI"),
            Opcodes::Srli => f.write_str("SRLI"),
            Opcodes::Srai => f.write_str("SRAI"),
            Opcodes::Slti => f.write_str("SLTI"),
            Opcodes::Sltiu => f.write_str("SLTIU"),
            Opcodes::Lb => f.write_str("LB"),
            Opcodes::Lh => f.write_str("LH"),
            Opcodes::Lw => f.write_str("LW"),
            Opcodes::Lbu => f.write_str("LBU"),
            Opcodes::Lhu => f.write_str("LHU"),
            Opcodes::Sb => f.write_str("SB"),
            Opcodes::Sh => f.write_str("SH"),
            Opcodes::Sw => f.write_str("SW"),
            Opcodes::Beq => f.write_str("BEQ"),
            Opcodes::Bne => f.write_str("BNE"),
            Opcodes::Blt => f.write_str("BLT"),
            Opcodes::Bge => f.write_str("BGE"),
            Opcodes::Bltu => f.write_str("BLTU"),
            Opcodes::Bgeu => f.write_str("BGEU"),
            Opcodes::Jal => f.write_str("JAL"),
            Opcodes::Jalr => f.write_str("JALR"),
            Opcodes::Lui => f.write_str("LUI"),
            Opcodes::Auipc => f.write_str("AUIPC"),
            Opcodes::Ecall => f.write_str("ECALL"),
            Opcodes::Ebreak => f.write_str("EBREAK"),
            Opcodes::Fence => f.write_str("FENCE"),
            Opcodes::Default => f.write_str("DEFAULT"),
        }
    }
}
