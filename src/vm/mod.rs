mod registers;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use instruction::Instruction;
use opcodes::Opcodes;
use registers::Registers;

use crate::elf_parser::{get_data_at_index, ElfHeader};

pub(crate) mod instruction;

mod opcodes;

pub(crate) const WORD_SIZE: usize = 4; // word size = 32 bits = 8bits * 4
const HALF_WORD: usize = 2;
const BYTE: usize = 1;
const MAX_ADDRESSABLE_MEMORY: usize = 1 << 32;
const TOTAL_REGISTERS: usize = 33;

struct Vm {
    running: bool,
    exit_code: u32,
    register: [u32; TOTAL_REGISTERS],
    memory: Vec<u8>,
}

impl Vm {
    fn initialize() -> Self {
        Self {
            running: false,
            exit_code: 0,
            register: [0; TOTAL_REGISTERS],
            memory: vec![0; MAX_ADDRESSABLE_MEMORY],
        }
    }

    fn fetch(&self) -> &[u8] {
        self.mem_read(self.get_register(Registers::Pc as u32))
    }

    fn load_program_from_file(&mut self, path: String) {
        let mut file = BufReader::new(File::open(path).unwrap());
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();

        let decoded_elf = ElfHeader::decode_elf(&buf);

        for ph in decoded_elf.e_ph {
            if ph.ph_type == 0x1 {
                if ph.file_size == 0 {
                    continue;
                };
                self.memory
                    [ph.virtual_address as usize..(ph.virtual_address + ph.memory_size) as usize]
                    .clone_from_slice(get_data_at_index(
                        &buf,
                        ph.offset as usize,
                        ph.file_size as usize,
                    ));
            }
        }

        self.set_register(Registers::Pc as u32, decoded_elf.e_entry);
    }

    fn run_program(&mut self) {
        self.running = true;

        while self.running {
            let instruction = self.fetch();

            let instr = Instruction::decode(instruction);

            self.execute(instr);
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            Opcodes::Add => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_add(self.get_register(instruction.rs2 as u32)),
                );
            }
            Opcodes::Sub => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_sub(self.get_register(instruction.rs2 as u32)),
                );
            }
            Opcodes::Xor => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        ^ self.get_register(instruction.rs2 as u32),
                );
            }
            Opcodes::Or => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        | self.get_register(instruction.rs2 as u32),
                );
            }
            Opcodes::And => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        & self.get_register(instruction.rs2 as u32),
                );
            }
            Opcodes::Sll => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_shl(self.get_register(instruction.rs2 as u32)),
                );
            }
            Opcodes::Srl => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_shr(self.get_register(instruction.rs2 as u32)),
                );
            }
            Opcodes::Sra => {
                let shift_value = self.get_register(instruction.rs2 as u32) & 0x1f;
                self.set_register(
                    instruction.rd as u32,
                    sign_extend(
                        self.get_register(instruction.rs1 as u32) >> shift_value,
                        31 - shift_value,
                    ),
                );
            }
            Opcodes::Slt => {
                self.set_register(
                    instruction.rd as u32,
                    if (self.get_register(instruction.rs1 as u32) as i32)
                        < (self.get_register(instruction.rs2 as u32) as i32)
                    {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Sltu => {
                self.set_register(
                    instruction.rd as u32,
                    if self.get_register(instruction.rs1 as u32)
                        < self.get_register(instruction.rs2 as u32)
                    {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Addi => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_add(instruction.imm),
                );
            }
            Opcodes::Xori => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32) ^ instruction.imm,
                );
            }
            Opcodes::Ori => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32) | instruction.imm,
                );
            }
            Opcodes::Andi => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32) & instruction.imm,
                );
            }
            Opcodes::Slli => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32) << instruction.imm,
                );
            }
            Opcodes::Srli => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(instruction.rs1 as u32) >> instruction.imm,
                );
            }
            Opcodes::Srai => {
                let shift_value = instruction.imm & 0x1f;
                self.set_register(
                    instruction.rd as u32,
                    sign_extend(
                        self.get_register(instruction.rs1 as u32)
                            .wrapping_shr(shift_value),
                        31 - shift_value,
                    ),
                );
            }
            Opcodes::Slti => {
                self.set_register(
                    instruction.rd as u32,
                    if (self.get_register(instruction.rs1 as u32) as i32) < instruction.imm as i32 {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Sltiu => {
                self.set_register(
                    instruction.rd as u32,
                    if self.get_register(instruction.rs1 as u32) < instruction.imm {
                        1
                    } else {
                        0
                    },
                );
            }
            Opcodes::Lb => {
                let mut value = self
                    .mem_read(
                        self.get_register(instruction.rs1 as u32)
                            .wrapping_add(instruction.imm),
                    )
                    .to_vec();

                value[1..=3].copy_from_slice(&[0, 0, 0]);

                self.set_register(
                    instruction.rd as u32,
                    sign_extend(u32::from_le_bytes(value.try_into().unwrap()), 7),
                );
            }
            Opcodes::Lh => {
                let mut value = self
                    .mem_read(
                        self.get_register(instruction.rs1 as u32)
                            .wrapping_add(instruction.imm),
                    )
                    .to_vec();

                value[2..=3].copy_from_slice(&[0, 0]);

                self.set_register(
                    instruction.rd as u32,
                    sign_extend(u32::from_le_bytes(value.try_into().unwrap()), 15),
                );
            }
            Opcodes::Lw => {
                self.set_register(
                    instruction.rd as u32,
                    u32::from_le_bytes(
                        self.mem_read(
                            self.get_register(instruction.rs1 as u32)
                                .wrapping_add(instruction.imm),
                        )
                        .try_into()
                        .unwrap(),
                    ),
                );
            }
            Opcodes::Lbu => {
                let mut value = self
                    .mem_read(
                        self.get_register(instruction.rs1 as u32)
                            .wrapping_add(instruction.imm),
                    )
                    .to_vec();

                value[1..=3].copy_from_slice(&[0, 0, 0]);

                self.set_register(
                    instruction.rd as u32,
                    u32::from_le_bytes(value.try_into().unwrap()),
                );
            }
            Opcodes::Lhu => {
                let mut value = self
                    .mem_read(
                        self.get_register(instruction.rs1 as u32)
                            .wrapping_add(instruction.imm),
                    )
                    .to_vec();

                value[2..=3].copy_from_slice(&[0, 0]);

                self.set_register(
                    instruction.rd as u32,
                    u32::from_le_bytes(value.try_into().unwrap()),
                );
            }
            Opcodes::Sb => {
                self.mem_write(
                    BYTE,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_add(instruction.imm),
                    &self
                        .get_register(instruction.rs2 as u32)
                        .to_le_bytes()
                        .as_slice()[0..1],
                );
            }
            Opcodes::Sh => {
                self.mem_write(
                    HALF_WORD,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_add(instruction.imm),
                    &self
                        .get_register(instruction.rs2 as u32)
                        .to_le_bytes()
                        .as_slice()[0..2],
                );
            }
            Opcodes::Sw => {
                self.mem_write(
                    WORD_SIZE,
                    self.get_register(instruction.rs1 as u32)
                        .wrapping_add(instruction.imm),
                    self.get_register(instruction.rs2 as u32)
                        .to_le_bytes()
                        .as_slice(),
                );
            }
            Opcodes::Beq => {
                if self.get_register(instruction.rs1 as u32)
                    == self.get_register(instruction.rs2 as u32)
                {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32)
                            .wrapping_add(instruction.imm),
                    );

                    return;
                }
            }
            Opcodes::Bne => {
                if self.get_register(instruction.rs1 as u32)
                    != self.get_register(instruction.rs2 as u32)
                {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32)
                            .wrapping_add(instruction.imm),
                    );

                    return;
                }
            }
            Opcodes::Blt => {
                if (self.get_register(instruction.rs1 as u32) as i32)
                    < (self.get_register(instruction.rs2 as u32) as i32)
                {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32)
                            .wrapping_add(instruction.imm),
                    );

                    return;
                }
            }
            Opcodes::Bge => {
                if (self.get_register(instruction.rs1 as u32) as i32)
                    >= (self.get_register(instruction.rs2 as u32) as i32)
                {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32)
                            .wrapping_add(instruction.imm),
                    );

                    return;
                }
            }
            Opcodes::Bltu => {
                if self.get_register(instruction.rs1 as u32)
                    < self.get_register(instruction.rs2 as u32)
                {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32)
                            .wrapping_add(instruction.imm),
                    );

                    return;
                }
            }
            Opcodes::Bgeu => {
                if self.get_register(instruction.rs1 as u32)
                    >= self.get_register(instruction.rs2 as u32)
                {
                    self.set_register(
                        Registers::Pc as u32,
                        self.get_register(Registers::Pc as u32)
                            .wrapping_add(instruction.imm),
                    );

                    return;
                }
            }

            Opcodes::Jal => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(Registers::Pc as u32) + 4,
                );
                self.set_register(
                    Registers::Pc as u32,
                    self.get_register(Registers::Pc as u32)
                        .wrapping_add(instruction.imm),
                );

                return;
            }
            Opcodes::Jalr => {
                let rs1 = self.get_register(instruction.rs1 as u32);

                self.set_register(
                    instruction.rd as u32,
                    self.get_register(Registers::Pc as u32) + 4,
                );
                self.set_register(Registers::Pc as u32, rs1.wrapping_add(instruction.imm));

                return;
            }
            Opcodes::Lui => {
                self.set_register(instruction.rd as u32, instruction.imm << 12);
            }
            Opcodes::Auipc => {
                self.set_register(
                    instruction.rd as u32,
                    self.get_register(Registers::Pc as u32)
                        .wrapping_add(instruction.imm << 12),
                );
            }
            Opcodes::Ecall => {
                // transfer control to Os
                match self.get_register(Registers::A7 as u32) {
                    63 => {
                        // read
                        let mut buf = vec![];
                        std::io::stdin().read(&mut buf).unwrap();
                        // self.mem_write(size, memory_address, value);
                        println!("Read from std in");
                    }
                    64 => {
                        // write
                        // let buf = self.mem_read(size, memory_address);
                        // std::io::stdout().write(buf);
                    }
                    93 => {
                        // exit
                        self.running = false;
                        self.exit_code = self.get_register(Registers::A0 as u32);
                        println!("Program halted");
                    }
                    _ => todo!(),
                }
            }
            Opcodes::Ebreak => todo!(), // transfer control to debugger
            Opcodes::Fence => {}        // order mem/io access

            //
            Opcodes::Default => panic!("Invalid opcode"),
            Opcodes::EAny => {
                // dbg!(format!("{}", instruction));
            }
        }

        self.update_pc();
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
        if register_address == 0 {
            return;
        }
        self.register[register_address as usize] = register_value;
    }

    fn mem_read(&self, memory_address: u32) -> &[u8] {
        &self.memory[memory_address as usize..(memory_address as usize + WORD_SIZE)]
    }

    fn mem_write(&mut self, size: usize, memory_address: u32, value: &[u8]) {
        self.memory[memory_address as usize..(memory_address as usize) + size]
            .copy_from_slice(&value);
    }
}

pub(crate) fn sign_extend(value: u32, bit_count: u32) -> u32 {
    if bit_count == 32 || (value >> bit_count) & 1 == 0 {
        value
    } else {
        (0xFFFFFFFF >> bit_count) << bit_count | value
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{Instruction, Vm, BYTE, HALF_WORD, WORD_SIZE};

    use super::sign_extend;

    #[test]
    fn test_mem_read() {
        let mut vm = Vm::initialize();

        let memory_address = 5;
        let value = [30, 15, 18, 20];

        vm.mem_write(WORD_SIZE, memory_address, &value);

        // read full word
        assert_eq!(vm.mem_read(memory_address), value);

        // store full word
        vm.mem_write(WORD_SIZE, memory_address, &[20, 18, 15, 30]);
        assert_eq!(vm.mem_read(memory_address), &[20, 18, 15, 30]);

        // store byte
        vm.mem_write(BYTE, memory_address, &[45]);
        assert_eq!(vm.mem_read(memory_address), &[45, 18, 15, 30]);

        // store half word
        vm.mem_write(HALF_WORD, memory_address, &[35, 40]);
        assert_eq!(vm.mem_read(memory_address), &[35, 40, 15, 30]);
    }

    #[test]
    fn test_rand() {
        // 0x00c58533 -> 1100  01011  000     01010  0110011 = add x10, x11, x12
        let mem = [0x00, 0xc5, 0x85, 0x33];
        let mut b = [0; WORD_SIZE];
        b[2..=3].clone_from_slice(&mem[2..=3]);
        let v = u32::from_le_bytes(mem);
        dbg!(format_args!("{:?}", b));
    }

    #[test]
    fn test_load_program_from_file() {
        for entry in std::fs::read_dir("src/examples/e2e-tests/").unwrap() {
            let mut vm = Vm::initialize();
            vm.load_program_from_file(String::from(entry.unwrap().path().to_str().unwrap()));
            vm.run_program();
            assert!(!vm.running);
            assert_eq!(vm.exit_code, 0);
        }
    }

    #[test]
    fn test_sign_extend() {
        let v = -6_i8;
        let v_ext = sign_extend(v as u32, 8);
        dbg!(v_ext as i32);
    }
}
