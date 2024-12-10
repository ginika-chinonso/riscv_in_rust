use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum Registers {
    Zero, // hard-wired zero
    Ra,   // return address
    Sp,   // stack pointer
    Gp,   // global pointer
    Tp,   // thread pointer
    T0,   // temporaries 0 - 2
    T1,
    T2,
    S0, // saved register zero / frame pointer
    S1, // saved register 1
    A0, // function arguments 0 - 7
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2, // saved registers 2 - 11
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3, // temporaries 3 - 6
    T4,
    T5,
    T6,
    Pc, // program counter

    Default,
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Registers::Zero => f.write_str("zro"),
            Registers::Ra => f.write_str("ra"),
            Registers::Sp => f.write_str("sp"),
            Registers::Gp => f.write_str("gp"),
            Registers::Tp => f.write_str("tp"),
            Registers::T0 => f.write_str("t0"),
            Registers::T1 => f.write_str("t1"),
            Registers::T2 => f.write_str("t2"),
            Registers::S0 => f.write_str("s0"),
            Registers::S1 => f.write_str("s1"),
            Registers::A0 => f.write_str("a0"),
            Registers::A1 => f.write_str("a1"),
            Registers::A2 => f.write_str("a2"),
            Registers::A3 => f.write_str("a3"),
            Registers::A4 => f.write_str("a4"),
            Registers::A5 => f.write_str("a5"),
            Registers::A6 => f.write_str("a6"),
            Registers::A7 => f.write_str("a7"),
            Registers::S2 => f.write_str("s2"),
            Registers::S3 => f.write_str("s3"),
            Registers::S4 => f.write_str("s4"),
            Registers::S5 => f.write_str("s5"),
            Registers::S6 => f.write_str("s6"),
            Registers::S7 => f.write_str("s7"),
            Registers::S8 => f.write_str("s8"),
            Registers::S9 => f.write_str("s9"),
            Registers::S10 => f.write_str("s10"),
            Registers::S11 => f.write_str("s11"),
            Registers::T3 => f.write_str("t3"),
            Registers::T4 => f.write_str("t4"),
            Registers::T5 => f.write_str("t5"),
            Registers::T6 => f.write_str("t6"),
            Registers::Pc => f.write_str("pc"),
            Registers::Default => f.write_str("default"),
        }
    }
}

impl Into<Registers> for u32 {
    fn into(self) -> Registers {
        if self == Registers::Zero as u32 {
            Registers::Zero
        } else if self == Registers::Ra as u32 {
            Registers::Ra
        } else if self == Registers::Sp as u32 {
            Registers::Sp
        } else if self == Registers::Gp as u32 {
            Registers::Gp
        } else if self == Registers::Tp as u32 {
            Registers::Tp
        } else if self == Registers::T0 as u32 {
            Registers::T0
        } else if self == Registers::T1 as u32 {
            Registers::T1
        } else if self == Registers::T2 as u32 {
            Registers::T2
        } else if self == Registers::S0 as u32 {
            Registers::S0
        } else if self == Registers::S1 as u32 {
            Registers::S1
        } else if self == Registers::A0 as u32 {
            Registers::A0
        } else if self == Registers::A1 as u32 {
            Registers::A1
        } else if self == Registers::A2 as u32 {
            Registers::A2
        } else if self == Registers::A3 as u32 {
            Registers::A3
        } else if self == Registers::A4 as u32 {
            Registers::A4
        } else if self == Registers::A5 as u32 {
            Registers::A5
        } else if self == Registers::A6 as u32 {
            Registers::A6
        } else if self == Registers::A7 as u32 {
            Registers::A7
        } else if self == Registers::S2 as u32 {
            Registers::S2
        } else if self == Registers::S3 as u32 {
            Registers::S3
        } else if self == Registers::S4 as u32 {
            Registers::S4
        } else if self == Registers::S5 as u32 {
            Registers::S5
        } else if self == Registers::S6 as u32 {
            Registers::S6
        } else if self == Registers::S7 as u32 {
            Registers::S7
        } else if self == Registers::S8 as u32 {
            Registers::S8
        } else if self == Registers::S9 as u32 {
            Registers::S9
        } else if self == Registers::S10 as u32 {
            Registers::S10
        } else if self == Registers::S11 as u32 {
            Registers::S11
        } else if self == Registers::T3 as u32 {
            Registers::T3
        } else if self == Registers::T4 as u32 {
            Registers::T4
        } else if self == Registers::T5 as u32 {
            Registers::T5
        } else if self == Registers::T6 as u32 {
            Registers::T6
        } else if self == Registers::Pc as u32 {
            Registers::Pc
        } else {
            Registers::Default
        }
    }
}
impl Into<Registers> for &str {
    fn into(self) -> Registers {
        if self == "zro" {
            Registers::Zero
        } else if self == "ra" {
            Registers::Ra
        } else if self == "sp" {
            Registers::Sp
        } else if self == "gp" {
            Registers::Gp
        } else if self == "tp" {
            Registers::Tp
        } else if self == "t0" {
            Registers::T0
        } else if self == "t1" {
            Registers::T1
        } else if self == "t2" {
            Registers::T2
        } else if self == "s0" {
            Registers::S0
        } else if self == "s1" {
            Registers::S1
        } else if self == "a0" {
            Registers::A0
        } else if self == "a1" {
            Registers::A1
        } else if self == "a2" {
            Registers::A2
        } else if self == "a3" {
            Registers::A3
        } else if self == "a4" {
            Registers::A4
        } else if self == "a5" {
            Registers::A5
        } else if self == "a6" {
            Registers::A6
        } else if self == "a7" {
            Registers::A7
        } else if self == "s2" {
            Registers::S2
        } else if self == "s3" {
            Registers::S3
        } else if self == "s4" {
            Registers::S4
        } else if self == "s5" {
            Registers::S5
        } else if self == "s6" {
            Registers::S6
        } else if self == "s7" {
            Registers::S7
        } else if self == "s8" {
            Registers::S8
        } else if self == "s9" {
            Registers::S9
        } else if self == "s10" {
            Registers::S10
        } else if self == "s11" {
            Registers::S11
        } else if self == "t3" {
            Registers::T3
        } else if self == "t4" {
            Registers::T4
        } else if self == "t5" {
            Registers::T5
        } else if self == "t6" {
            Registers::T6
        } else if self == "pc" {
            Registers::Pc
        } else {
            Registers::Default
        }
    }
}

// Floating point registers
enum FPRegisters {
    F0, // fp temporaries 0 - 7
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8, // fp saved registers 8 - 9
    F9,
    F10, // fp args/return value 10 - 11
    F11,
    F12, // fp args 12 - 17
    F13,
    F14,
    F15,
    F16,
    F17,
    F18, // fp saved registers 18 - 27
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28, // fp temporaries 28 - 31
    F29,
    F30,
    F31,
}
