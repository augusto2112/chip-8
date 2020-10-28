use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Register {
    pub id: u8,
}

impl Register {
    pub fn new(id: u16) -> Register {
        if id > 15 {
            panic!("Trying to create register with id {}", id);
        }
        Self { id: id as u8 }
    }

    pub fn vf() -> Register {
        Self::new(0xf)
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{:x}", self.id)
    }
}

pub struct Registers {
    registers: [u8; 16],
    pub i: u16,
    pub dt: u8,
    pub st: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn read(&self, register: &Register) -> u8 {
        self.registers[register.id as usize]
    }

    pub fn write(&mut self, register: &Register, value: u8) {
        self.registers[register.id as usize] = value
    }

    pub fn tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Registers:")?;
        for (i, value) in self.registers.iter().enumerate() {
            write!(f, "{}: {} \t", Register::new(i as u16), value)?;
        }
        writeln!(f)?;
        write!(f, "I: {} \t", self.i)?;
        write!(f, "DT: {} \t", self.dt)?;
        write!(f, "ST: {} \t", self.st)
    }
}
