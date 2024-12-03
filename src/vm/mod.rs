mod registers;
use std::{fs::File, io::Read};

use instruction::{into_byte, into_u32, Instruction};
use opcodes::Opcodes;
use registers::Registers;

mod instruction;

mod opcodes;

const WORD_SIZE: usize = 4; // word size = 32 bits = 8bits * 4
const HALF_WORD: usize = 2;
const BYTE: usize = 1;
const MAX_ADDRESSABLE_MEMORY: usize = 1 << 32; // ????
const TOTAL_REGISTERS: usize = 33;

struct Vm {
    register: [u32; TOTAL_REGISTERS],
    memory: Vec<u8>,
}

// TODO: work on sign extension

impl Vm {
    fn initialize() -> Self {
        Self {
            register: [0; TOTAL_REGISTERS],
            memory: vec![0; MAX_ADDRESSABLE_MEMORY],
        }
    }

    fn fetch(&self) -> &[u8] {
        self.mem_read(WORD_SIZE, Registers::Pc as u32)
    }

    fn load_program_from_file(&mut self, path: String) {
        let mut file = File::open(path).unwrap();
        let mut buf = vec![];
        file.read(&mut buf).unwrap();
        // self.memory[self.get_register(Registers::Pc as u32)..self.get_register(Registers::Pc as u32) + buf.len() as u32].clone_from_slice(&buf);
    }

    fn run_program(&mut self) {
        let instruction = self.fetch();
        let instr = Instruction::decode(instruction);
        self.execute(instr);
        self.update_pc();
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            Opcodes::Add => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) + self.get_register(instruction.rs2),
                );
            }
            Opcodes::Sub => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) - self.get_register(instruction.rs2),
                );
            }
            Opcodes::Xor => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) ^ self.get_register(instruction.rs2),
                );
            }
            Opcodes::Or => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) | self.get_register(instruction.rs2),
                );
            }
            Opcodes::And => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) & self.get_register(instruction.rs2),
                );
            }
            Opcodes::Sll => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) << self.get_register(instruction.rs2),
                );
            }
            Opcodes::Srl => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) >> self.get_register(instruction.rs2),
                );
            }
            Opcodes::Sra => {
                // todo: extend msb
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) >> self.get_register(instruction.rs2),
                );
            }
            Opcodes::Slt => {
                self.set_register(
                    instruction.rd,
                    if self.get_register(instruction.rs1) < self.get_register(instruction.rs2) {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Sltu => {
                // todo: zero extend
                self.set_register(
                    instruction.rd,
                    if self.get_register(instruction.rs1) < self.get_register(instruction.rs2) {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Addi => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) + instruction.imm,
                );
            }
            Opcodes::Xori => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) ^ instruction.imm,
                );
            }
            Opcodes::Ori => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) | instruction.imm,
                );
            }
            Opcodes::Andi => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) & instruction.imm,
                );
            }
            Opcodes::Slli => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) << instruction.imm,
                );
            }
            Opcodes::Srli => {
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) >> instruction.imm,
                );
            }
            Opcodes::Srai => {
                // todo: extend msb
                self.set_register(
                    instruction.rd,
                    self.get_register(instruction.rs1) >> instruction.imm,
                );
            }
            Opcodes::Slti => {
                self.set_register(
                    instruction.rd,
                    if self.get_register(instruction.rs1) < instruction.imm {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Sltiu => {
                // todo: extend zero
                self.set_register(
                    instruction.rd,
                    if self.get_register(instruction.rs1) < instruction.imm {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Lb => {
                self.set_register(
                    instruction.rd,
                    into_u32(
                        self.mem_read(BYTE, self.get_register(instruction.rs1) + instruction.imm),
                    ),
                );
            }
            Opcodes::Lh => {
                self.set_register(
                    instruction.rd,
                    into_u32(self.mem_read(
                        HALF_WORD,
                        self.get_register(instruction.rs1) + instruction.imm,
                    )),
                );
            }
            Opcodes::Lw => {
                self.set_register(
                    instruction.rd,
                    into_u32(self.mem_read(
                        WORD_SIZE,
                        self.get_register(instruction.rs1) + instruction.imm,
                    )),
                );
            }
            Opcodes::Lbu => {
                // zero extend
                self.set_register(
                    instruction.rd,
                    into_u32(
                        self.mem_read(BYTE, self.get_register(instruction.rs1) + instruction.imm),
                    ),
                );
            }
            Opcodes::Lhu => {
                // zero extend
                self.set_register(
                    instruction.rd,
                    into_u32(self.mem_read(
                        HALF_WORD,
                        self.get_register(instruction.rs1) + instruction.imm,
                    )),
                );
            }
            Opcodes::Sb => {
                self.mem_write(
                    BYTE,
                    self.get_register(instruction.rs1) + instruction.imm,
                    into_byte(instruction.rs2).as_slice(),
                );
            }
            Opcodes::Sh => {
                self.mem_write(
                    HALF_WORD,
                    self.get_register(instruction.rs1) + instruction.imm,
                    into_byte(instruction.rs2).as_slice(),
                );
            }
            Opcodes::Sw => {
                self.mem_write(
                    WORD_SIZE,
                    self.get_register(instruction.rs1) + instruction.imm,
                    into_byte(instruction.rs2).as_slice(),
                );
            }
            Opcodes::Beq => {
                if instruction.rs1 == instruction.rs2 {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32) + instruction.imm,
                    );
                }
            }
            Opcodes::Bne => {
                if instruction.rs1 != instruction.rs2 {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32) + instruction.imm,
                    );
                }
            }
            Opcodes::Blt => {
                if instruction.rs1 < instruction.rs2 {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32) + instruction.imm,
                    );
                }
            }
            Opcodes::Bge => {
                if instruction.rs1 >= instruction.rs2 {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32) + instruction.imm,
                    );
                }
            }
            Opcodes::Bltu => {
                // todo: zero extend
                if instruction.rs1 < instruction.rs2 {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32) + instruction.imm,
                    );
                }
            }
            Opcodes::Bgeu => {
                // todo: zero extend
                if instruction.rs1 >= instruction.rs2 {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32) + instruction.imm,
                    );
                }
            }

            Opcodes::Jal => {
                self.set_register(instruction.rd, self.get_register(Registers::Pc as u32) + 4);
                self.set_register(
                    Registers::Pc as u32,
                    self.get_register(Registers::Pc as u32) + instruction.imm,
                );
            }
            Opcodes::Jalr => {
                self.set_register(instruction.rd, self.get_register(Registers::Pc as u32) + 4);
                self.set_register(
                    Registers::Pc as u32,
                    self.get_register(instruction.rs1) + instruction.imm,
                );
            }
            Opcodes::Lui => {
                self.set_register(instruction.rd, instruction.imm >> 12);
            }
            Opcodes::Auipc => {
                self.set_register(
                    instruction.rd,
                    self.get_register(Registers::Pc as u32) + (instruction.imm >> 12),
                );
            }
            Opcodes::Ecall => todo!(),  // transfer control to Os
            Opcodes::Ebreak => todo!(), // transfer control to debugger
            Opcodes::Fence => todo!(),  // order mem/io access
            Opcodes::Default => panic!("Invalid opcode"),
        }
    }

    fn update_pc(&mut self) {
        self.set_register(
            Registers::Pc as u32,
            self.get_register(Registers::Pc as u32) + WORD_SIZE as u32,
        );
    }

    fn get_register(&self, register_address: u32) -> u32 {
        self.register[register_address as usize]
    }

    fn set_register(&mut self, register_address: u32, register_value: u32) {
        self.register[register_address as usize] = register_value;
    }

    fn mem_read(&self, size: usize, memory_address: u32) -> &[u8] {
        // todo: verify if this should be be or le???
        &self.memory
            [((memory_address as usize) + WORD_SIZE - size)..(memory_address as usize + WORD_SIZE)]
    }

    fn mem_write(&mut self, size: usize, memory_address: u32, value: &[u8]) {
        // todo: verify if this should be be or le???
        self.memory[((memory_address as usize) + WORD_SIZE - size)
            ..((memory_address as usize) + WORD_SIZE)]
            .copy_from_slice(&value);
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{BYTE, HALF_WORD, WORD_SIZE};

    use super::{instruction::into_u32, Vm};

    #[test]
    fn test_mem_read() {
        let mut vm = Vm::initialize();

        let memory_address = 5;
        let value = [30, 15, 18, 20];

        vm.mem_write(WORD_SIZE, memory_address, &value);

        // read full word
        assert_eq!(vm.mem_read(WORD_SIZE, memory_address), value);

        //read byte
        assert_eq!(vm.mem_read(1, memory_address), [20]);

        //read half word
        assert_eq!(vm.mem_read(2, memory_address), [18, 20]);

        // store full word
        vm.mem_write(WORD_SIZE, 5, &[20, 18, 15, 30]);
        assert_eq!(vm.mem_read(WORD_SIZE, 5), &[20, 18, 15, 30]);

        // store byte
        vm.mem_write(BYTE, 5, &[45]);
        assert_eq!(vm.mem_read(WORD_SIZE, 5), &[20, 18, 15, 45]);

        // store half word
        vm.mem_write(HALF_WORD, 5, &[20, 60]);
        assert_eq!(vm.mem_read(WORD_SIZE, 5), &[20, 18, 20, 60]);
    }

    #[test]
    fn test_rand() {
        // 0x00c58533 -> 1100  01011  000     01010  0110011 = add x10, x11, x12
        let mem = [0x00, 0xc5, 0x85, 0x33];
        let mut b = [0; WORD_SIZE];
        b[2..=3].clone_from_slice(&mem[2..=3]);
        let v = into_u32(&mem);
        dbg!(format_args!("{:?}", b));
    }
}
