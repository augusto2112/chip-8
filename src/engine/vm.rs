use super::cpu::CPU;
use super::key::Key;
use super::memory::Memory;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const PROGRAM_OFFSET: u16 = 0x200;

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct VM {
    cpu: CPU,
    memory: Memory,
    frame_buffer: Vec<u32>,
}

impl VM {
    pub fn new(rom: &[u8]) -> VM {
        let mut memory = Memory::new();
        memory.load(rom, PROGRAM_OFFSET);
        memory.load(&FONT_SET, 0);
        VM {
            cpu: CPU::new(),
            memory,
            frame_buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn get_current_frame(&self) -> Vec<u32> {
        self.frame_buffer.clone()
    }

    pub fn tick(&mut self, keys: &[Key]) {
        self.cpu
            .tick(&mut self.memory, &mut self.frame_buffer, keys)
    }
}