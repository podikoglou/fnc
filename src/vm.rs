use std::io::Read;

use crate::{HEIGHT, WIDTH};

const FONT: [u8; 80] = [
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

#[derive(Debug)]
pub struct VM {
    memory: [u8; 4096],

    registers: [u8; 16],
    address_regiser: u16,
    pc: usize,

    stack: Vec<u16>,

    buffer: Vec<u32>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0x00; 4096],
            registers: [0x00; 16],
            address_regiser: 0x00,
            pc: 0,
            stack: vec![],
            buffer: vec![0u32; WIDTH * HEIGHT],
        }
    }

    pub fn render(&mut self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn load(&self, reader: impl Read) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn load_font(&mut self) {
        for (idx, val) in FONT.iter().enumerate() {
            self.memory[idx] = *val;
        }
    }

    pub fn execute_instruction(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => self.clear_screen(),
                    0x00EE => { /* return */ }
                    other => todo!("unimplemented opcode: {}", other),
                }
            }

            0x1000 => { /* jump to address */ }
            0x2000 => { /* call subroutine */ }
            0x3000 => { /* skip if Vx == NN */ }
            0x4000 => { /* skip if Vx != NN */ }
            0x5000 => { /* skip if Vx == Vy */ }
            0x6000 => { /* Vx = NN */ }
            0x7000 => { /* Vx += NN */ }

            other => todo!("unimplemented opcode: {}", other),
        }
    }

    fn clear_screen(&mut self) {
        self.buffer = vec![0u32; WIDTH * HEIGHT];
    }
}
