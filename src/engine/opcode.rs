use super::register::Register;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Opcode {
    SYS(u16),                    // 0nnn - SYS addr
    CLS,                         // 00E0 - CLS
    RET,                         // 00EE - RET
    JP(u16),                     // 1nnn - JP addr
    CALL(u16),                   // 2nnn - CALL addr
    SEI(Register, u8),           // 3xkk - SE Vx, byte
    SNEI(Register, u8),          // 4xkk - SNE Vx, byte
    SE(Register, Register),      // 5xy0 - SE Vx, Vy
    LDI(Register, u8),           // 6xkk - LD Vx, byte
    ADDI(Register, u8),          // 7xkk - ADD Vx, byte
    LD(Register, Register),      // 8xy0 - LD Vx, Vy
    OR(Register, Register),      // 8xy1 - OR Vx, Vy
    AND(Register, Register),     // 8xy2 - AND Vx, Vy
    XOR(Register, Register),     // 8xy3 - XOR Vx, Vy
    ADD(Register, Register),     // 8xy4 - ADD Vx, Vy
    SUBR(Register, Register),    // 8xy5 - SUB Vx,
    SHR(Register),               // 8xy6 - SHR Vx {, Vy}
    SUBN(Register, Register),    // 8xy7 - SUBN Vx, Vy
    SHL(Register),               // 8xyE - SHL Vx {, Vy}
    SNE(Register, Register),     // 9xy0 - SNE Vx, Vy
    LDII(u16),                   // Annn - LD I, addr
    JPA(u16),                    // Bnnn - JP V0, addr
    RND(Register, u8),           // Cxkk - RND Vx, byte
    DRW(Register, Register, u8), // Dxyn - DRW Vx, Vy, nibble
    SKP(Register),               // Ex9E - SKP Vx
    SKNP(Register),              // ExA1 - SKNP Vx
    LDVDT(Register),             // Fx07 - LD Vx, DT
    LDK(Register),               // Fx0A - LD Vx, K
    LDDTV(Register),             // Fx15 - LD DT, Vx
    LDST(Register),              // Fx18 - LD ST, Vx
    ADDRI(Register),             // Fx1E - ADD I, Vx
    LDF(Register),               // Fx29 - LD F, Vx
    LDB(Register),               // Fx33 - LD B, Vx
    LDIM(Register),              // Fx55 - LD [I], Vx
    LDMI(Register),              // Fx65 - LD Vx, [I]
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::SYS(addr) => write!(f, "SYS \t{}", addr),
            Opcode::CLS => write!(f, "CLS"),
            Opcode::RET => write!(f, "RET"),
            Opcode::JP(addr) => write!(f, "JP \t\t{}", addr),
            Opcode::CALL(addr) => write!(f, "CALL \t{}", addr),
            Opcode::SEI(reg, byte) => write!(f, "SEI \tR{:x} \t{}", reg.id, byte),
            Opcode::SNEI(reg, byte) => write!(f, "SNEI \tR{:x} \t{}", reg.id, byte),
            Opcode::SE(reg1, reg2) => write!(f, "SE \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::LDI(reg, byte) => write!(f, "LDI \tR{:x} \t{}", reg.id, byte),
            Opcode::ADDI(reg, byte) => write!(f, "ADDI \tR{:x} \t{}", reg.id, byte),
            Opcode::LD(reg1, reg2) => write!(f, "LD \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::OR(reg1, reg2) => write!(f, "OR \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::AND(reg1, reg2) => write!(f, "AND \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::XOR(reg1, reg2) => write!(f, "XOR \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::ADD(reg1, reg2) => write!(f, "ADD \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::SUBR(reg1, reg2) => write!(f, "SUBR \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::SHR(reg) => write!(f, "SHR \tR{:x}", reg.id),
            Opcode::SUBN(reg1, reg2) => write!(f, "SUBN \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::SHL(reg) => write!(f, "SHL \tR{:x}", reg.id),
            Opcode::SNE(reg1, reg2) => write!(f, "SNE \tR{:x} \tR{:x}", reg1.id, reg2.id),
            Opcode::LDII(addr) => write!(f, "LDII \t{}", addr),
            Opcode::JPA(addr) => write!(f, "JPA \t{}", addr),
            Opcode::RND(reg, byte) => write!(f, "RND \tR{:x} \t{}", reg.id, byte),
            Opcode::DRW(reg1, reg2, byte) => {
                write!(f, "DRW  \tR{:x}  \tR{:x} \t{}", reg1.id, reg2.id, byte)
            }
            Opcode::SKP(reg) => write!(f, "SKP \tR{:x}", reg.id),
            Opcode::SKNP(reg) => write!(f, "SKNP \tR{:x}", reg.id),
            Opcode::LDVDT(reg) => write!(f, "LDVDT \tR{:x}", reg.id),
            Opcode::LDK(reg) => write!(f, "LDK \tR{:x}", reg.id),
            Opcode::LDDTV(reg) => write!(f, "LDDTV \tR{:x}", reg.id),
            Opcode::LDST(reg) => write!(f, "LDST \tR{:x}", reg.id),
            Opcode::ADDRI(reg) => write!(f, "ADDRI \tR{:x}", reg.id),
            Opcode::LDF(reg) => write!(f, "LDF \tR{:x}", reg.id),
            Opcode::LDB(reg) => write!(f, "LDB \tR{:x}", reg.id),
            Opcode::LDIM(reg) => write!(f, "LDIM \tR{:x}", reg.id),
            Opcode::LDMI(reg) => write!(f, "LDMI \tR{:x}", reg.id),
        }
    }
}
