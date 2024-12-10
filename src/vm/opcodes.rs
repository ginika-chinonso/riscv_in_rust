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
    EAny,
}

pub(crate) enum OpcodeTypes {
    RType = 0x33,
    ImmType = 0x13,
    LoadType = 0x3,
    SType = 0x23,
    BType = 0x63,
    Lui = 0x37,
    Auipc = 0x17,
    JType = 0x6f,
    Jalr = 0x67,
    Ecalls = 0x73,
    Fence = 0xF,
}

impl Display for Opcodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcodes::Add => f.write_str("add"),
            Opcodes::Sub => f.write_str("sub"),
            Opcodes::Xor => f.write_str("xor"),
            Opcodes::Or => f.write_str("or"),
            Opcodes::And => f.write_str("and"),
            Opcodes::Sll => f.write_str("sll"),
            Opcodes::Srl => f.write_str("srl"),
            Opcodes::Sra => f.write_str("sra"),
            Opcodes::Slt => f.write_str("slt"),
            Opcodes::Sltu => f.write_str("sltu"),
            Opcodes::Addi => f.write_str("addi"),
            Opcodes::Xori => f.write_str("xori"),
            Opcodes::Ori => f.write_str("ori"),
            Opcodes::Andi => f.write_str("andi"),
            Opcodes::Slli => f.write_str("slli"),
            Opcodes::Srli => f.write_str("srli"),
            Opcodes::Srai => f.write_str("srai"),
            Opcodes::Slti => f.write_str("slti"),
            Opcodes::Sltiu => f.write_str("sltiu"),
            Opcodes::Lb => f.write_str("lb"),
            Opcodes::Lh => f.write_str("lh"),
            Opcodes::Lw => f.write_str("lw"),
            Opcodes::Lbu => f.write_str("lbu"),
            Opcodes::Lhu => f.write_str("lhu"),
            Opcodes::Sb => f.write_str("sb"),
            Opcodes::Sh => f.write_str("sh"),
            Opcodes::Sw => f.write_str("sw"),
            Opcodes::Beq => f.write_str("beq"),
            Opcodes::Bne => f.write_str("bne"),
            Opcodes::Blt => f.write_str("blt"),
            Opcodes::Bge => f.write_str("bge"),
            Opcodes::Bltu => f.write_str("bltu"),
            Opcodes::Bgeu => f.write_str("bgeu"),
            Opcodes::Jal => f.write_str("jal"),
            Opcodes::Jalr => f.write_str("jalr"),
            Opcodes::Lui => f.write_str("lui"),
            Opcodes::Auipc => f.write_str("auipc"),
            Opcodes::Ecall => f.write_str("ecall"),
            Opcodes::Ebreak => f.write_str("ebreak"),
            Opcodes::Fence => f.write_str("fence"),
            //
            Opcodes::Default => f.write_str("default"),
            Opcodes::EAny => f.write_str("eany"),
        }
    }
}
