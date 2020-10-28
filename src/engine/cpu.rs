use super::key::Key;
use super::memory::Memory;
use super::opcode::Opcode;
use super::register::{Register, Registers};
use super::vm::{HEIGHT, PROGRAM_OFFSET, WIDTH};

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};

pub struct CPU {
    registers: Registers,
    program_counter: u16,
    stack: Vec<u16>,
    rng: ThreadRng,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            program_counter: PROGRAM_OFFSET,
            stack: vec![],
            rng: thread_rng(),
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, frame_buffer: &mut Vec<u32>, keys: &[Key]) {
        let encoded_instruction = self.fetch(memory);
        let opcode = CPU::decode(encoded_instruction);
        self.execute(opcode, memory, frame_buffer, keys);
        self.registers.tick()
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        (memory.read(self.program_counter) as u16) << 8
            | memory.read(self.program_counter + 1) as u16
    }

    fn decode(instruction: u16) -> Opcode {
        match instruction >> 12 {
            0x0 => match instruction {
                0x00e0 => Opcode::CLS,
                0x00ee => Opcode::RET,
                _ => Opcode::SYS(instruction & 0xfff),
            },
            0x1 => Opcode::JP(instruction & 0xfff),
            0x2 => Opcode::CALL(instruction & 0xfff),
            0x3 => Opcode::SEI(
                Register::new((instruction >> 8) & 0xf),
                (instruction & 0x0ff) as u8,
            ),
            0x4 => Opcode::SNEI(
                Register::new((instruction >> 8) & 0xf),
                (instruction & 0x0ff) as u8,
            ),
            0x5 => Opcode::SE(
                Register::new((instruction >> 8) & 0xf),
                Register::new((instruction >> 4) & 0xf),
            ),
            0x6 => Opcode::LDI(
                Register::new((instruction >> 8) & 0xf),
                (instruction & 0x0ff) as u8,
            ),
            0x7 => Opcode::ADDI(
                Register::new((instruction >> 8) & 0xf),
                (instruction & 0x0ff) as u8,
            ),
            0x8 => match instruction & 0xf {
                0x0 => Opcode::LD(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0x1 => Opcode::OR(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0x2 => Opcode::AND(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0x3 => Opcode::XOR(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0x4 => Opcode::ADD(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0x5 => Opcode::SUBR(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0x6 => Opcode::SHR(Register::new((instruction >> 8) & 0xf)),
                0x7 => Opcode::SUBN(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                ),
                0xe => Opcode::SHL(Register::new((instruction >> 8) & 0xf)),
                _ => panic!("{:#06x}", instruction),
            },
            0x9 => {
                if instruction & 0xf != 0x0 {
                    panic!("Instruction {} (SNE Vx Vy) should end with 0", instruction);
                }
                Opcode::SNE(
                    Register::new((instruction >> 8) & 0xf),
                    Register::new((instruction >> 4) & 0xf),
                )
            }
            0xa => Opcode::LDII(instruction & 0x0fff),
            0xb => Opcode::JPA(instruction & 0x0fff),
            0xc => Opcode::RND(
                Register::new((instruction >> 8) & 0xf),
                (instruction & 0x0ff) as u8,
            ),
            0xd => Opcode::DRW(
                Register::new((instruction >> 8) & 0xf),
                Register::new((instruction >> 4) & 0xf),
                (instruction & 0x000f) as u8, // n
            ),
            0xe => match instruction & 0xff {
                0x9e => Opcode::SKP(Register::new((instruction >> 8) & 0xf)),
                0xa1 => Opcode::SKNP(Register::new((instruction >> 8) & 0xf)),
                _ => panic!("{:#06x}", instruction),
            },
            0xf => match instruction & 0xff {
                0x07 => Opcode::LDVDT(Register::new((instruction >> 8) & 0xf)),
                0x0a => Opcode::LDK(Register::new((instruction >> 8) & 0xf)),
                0x15 => Opcode::LDDTV(Register::new((instruction >> 8) & 0xf)),
                0x18 => Opcode::LDST(Register::new((instruction >> 8) & 0xf)),
                0x1e => Opcode::ADDRI(Register::new((instruction >> 8) & 0xf)),
                0x29 => Opcode::LDF(Register::new((instruction >> 8) & 0xf)),
                0x33 => Opcode::LDB(Register::new((instruction >> 8) & 0xf)),
                0x55 => Opcode::LDIM(Register::new((instruction >> 8) & 0xf)),
                0x65 => Opcode::LDMI(Register::new((instruction >> 8) & 0xf)),
                _ => panic!("{:#06x}", instruction),
            },
            _ => panic!("{:#06x}", instruction),
        }
    }

    fn execute(
        &mut self,
        opcode: Opcode,
        memory: &mut Memory,
        frame_buffer: &mut Vec<u32>,
        keys: &[Key],
    ) {
        match opcode {
            Opcode::SYS(_) => {} // unimplemented in modern interpreters?
            Opcode::CLS => frame_buffer.iter_mut().for_each(|x| *x = 0),
            Opcode::RET => {
                self.program_counter = self.stack.pop().unwrap();
            }
            Opcode::JP(addr) => {
                self.program_counter = addr;
                return;
            }
            Opcode::CALL(addr) => {
                self.stack.push(self.program_counter);
                self.program_counter = addr;
                return;
            }
            Opcode::SEI(register, byte) => {
                if self.registers.read(&register) == byte {
                    self.program_counter += 2;
                }
            }
            Opcode::SNEI(register, byte) => {
                if self.registers.read(&register) != byte {
                    self.program_counter += 2;
                }
            }
            Opcode::SE(register_1, register_2) => {
                if self.registers.read(&register_1) == self.registers.read(&register_2) {
                    self.program_counter += 2;
                }
            }
            Opcode::LDI(register, value) => self.registers.write(&register, value),
            Opcode::ADDI(register, value) => {
                let updated = self.registers.read(&register).wrapping_add(value);
                self.registers.write(&register, updated);
            }
            Opcode::LD(register_1, register_2) => {
                let value = self.registers.read(&register_2);
                self.registers.write(&register_1, value)
            }
            Opcode::OR(register_1, register_2) => {
                let value_1 = self.registers.read(&register_1);
                let value_2 = self.registers.read(&register_2);
                self.registers.write(&register_1, value_1 | value_2)
            }
            Opcode::AND(register_1, register_2) => {
                let value_1 = self.registers.read(&register_1);
                let value_2 = self.registers.read(&register_2);
                self.registers.write(&register_1, value_1 & value_2)
            }
            Opcode::XOR(register_1, register_2) => {
                let value_1 = self.registers.read(&register_1);
                let value_2 = self.registers.read(&register_2);
                self.registers.write(&register_1, value_1 ^ value_2)
            }
            Opcode::ADD(register_1, register_2) => {
                let value_1 = self.registers.read(&register_1) as u16;
                let value_2 = self.registers.read(&register_2) as u16;
                let (sum, overflow) = value_1.overflowing_add(value_2);
                self.registers.write(&register_1, sum as u8);
                self.registers
                    .write(&Register::vf(), if overflow { 1 } else { 0 })
            }
            Opcode::SUBR(register_1, register_2) => {
                let value_1 = self.registers.read(&register_1);
                let value_2 = self.registers.read(&register_2);
                let (value, overflow) = value_1.overflowing_sub(value_2);
                self.registers
                    .write(&Register::vf(), if overflow { 0 } else { 1 });
                self.registers.write(&register_1, value)
            }
            Opcode::SHR(register) => {
                let value = self.registers.read(&register);
                self.registers.write(&Register::vf(), value & 0x1);
                self.registers.write(&register, value >> 1)
            }
            Opcode::SUBN(register_1, register_2) => {
                let value_1 = self.registers.read(&register_1);
                let value_2 = self.registers.read(&register_2);
                let (value, overflow) = value_2.overflowing_sub(value_1);
                self.registers
                    .write(&Register::vf(), if overflow { 0 } else { 1 });
                self.registers.write(&register_1, value)
            }
            Opcode::SHL(register) => {
                let value = self.registers.read(&register);
                self.registers.write(&Register::vf(), (value >> 7) & 0x1);
                self.registers.write(&register, value << 1)
            }
            Opcode::SNE(register_1, register_2) => {
                if self.registers.read(&register_1) != self.registers.read(&register_2) {
                    self.program_counter += 2;
                }
            }
            Opcode::LDII(nnn) => self.registers.i = nnn,
            Opcode::JPA(addr) => {
                let value = self.registers.read(&Register::new(0));
                self.program_counter = addr.wrapping_add(value as u16);
                return;
            }
            Opcode::RND(register, byte) => {
                let value: u8 = self.rng.gen();
                self.registers.write(&register, value & byte);
            }
            Opcode::DRW(x_register, y_register, n) => {
                let x_offset = self.registers.read(&x_register);
                let y_offset = self.registers.read(&y_register);
                let mut changed = 0;
                for ys in 0..n {
                    let line = memory.read(self.registers.i + ys as u16);
                    for xs in 0..8 {
                        if (line & (0x80 >> xs)) != 0 {
                            let x = (x_offset as usize + xs as usize) % WIDTH;
                            let y = (y_offset as usize + ys as usize) % HEIGHT;

                            let l = y * WIDTH + x;
                            if frame_buffer[l] != 0 {
                                changed = 1
                            }
                            frame_buffer[l] = !frame_buffer[l];
                        }
                    }
                }
                self.registers.write(&Register::vf(), changed);
            }
            Opcode::SKP(register) => {
                let value = self.registers.read(&register);
                if keys.iter().any(|x| (*x as u8) == value) {
                    self.program_counter += 2;
                }
            }
            Opcode::SKNP(register) => {
                let value = self.registers.read(&register);
                if keys.iter().all(|x| (*x as u8) != value) {
                    self.program_counter += 2;
                }
            }
            Opcode::LDVDT(register) => self.registers.write(&register, self.registers.dt),
            Opcode::LDK(register) => {
                if keys.is_empty() {
                    return;
                }
                self.registers
                    .write(&register, *keys.first().unwrap() as u8)
            }
            Opcode::LDDTV(register) => self.registers.dt = self.registers.read(&register),
            Opcode::LDST(register) => self.registers.st = self.registers.read(&register),
            Opcode::ADDRI(register) => {
                self.registers.i = self
                    .registers
                    .i
                    .wrapping_add(self.registers.read(&register) as u16)
            }
            Opcode::LDF(register) => self.registers.i = (self.registers.read(&register) * 5) as u16,
            Opcode::LDB(register) => {
                let value = self.registers.read(&register);
                let first = value / 100;
                let second = (value % 100) / 10;
                let third = value % 10;
                memory.write(self.registers.i, first);
                memory.write(self.registers.i + 1, second);
                memory.write(self.registers.i + 2, third)
            }
            Opcode::LDIM(register) => {
                let id = register.id;
                for i in 0..=id {
                    let register = Register::new(i as u16);
                    let value = self.registers.read(&register);
                    memory.write(self.registers.i + i as u16, value);
                }
            }
            Opcode::LDMI(register) => {
                let id = register.id;
                for i in 0..=id {
                    let register = Register::new(i as u16);
                    let value = memory.read(self.registers.i + i as u16);
                    self.registers.write(&register, value);
                }
            }
        }
        self.program_counter += 2;
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.registers)?;
        writeln!(f)?;
        writeln!(f, "PC: {}", self.program_counter)?;
        writeln!(f, "Stack:")?;
        for v in self.stack.iter() {
            write!(f, "{}\t", v)?;
        }
        writeln!(f)
    }
}
