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
