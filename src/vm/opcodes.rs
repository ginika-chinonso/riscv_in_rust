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
