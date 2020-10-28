use super::vm::PROGRAM_OFFSET;

const MEMORY_LENGTH: usize = 0xfff;

pub struct Memory {
    memory: [u8; MEMORY_LENGTH],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_LENGTH],
        }
    }

    pub fn load(&mut self, data: &[u8], offset: u16) {
        for (address, value) in data.iter().enumerate() {
            self.memory[offset as usize + address as usize] = *value
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < PROGRAM_OFFSET {
            panic!("ROM writing to read only data!");
        }
        self.memory[address as usize] = value
    }
}
