use std::io::Read;

use anyhow::bail;

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
    index_register: u16,
    pc: usize,
    instruction: u16,

    stack: Vec<u16>,

    buffer: Vec<u32>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0x00; 4096],
            registers: [0x00; 16],
            address_regiser: 0x00,
            index_register: 0x00,
            pc: 0,
            instruction: 0x00,
            stack: vec![],
            buffer: vec![0u32; WIDTH * HEIGHT],
        }
    }

    pub fn render(&mut self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn load(&mut self, mut reader: impl Read) -> anyhow::Result<()> {
        let mut buf: [u8; 64] = [0x00; 64];

        let mut pos = 512;

        loop {
            match reader.read(&mut buf) {
                Ok(read) => {
                    if read == 0 {
                        break;
                    }
                }
                Err(err) => bail!(err),
            };

            for byte in buf {
                self.memory[pos] = byte;
                pos += 1
            }
        }

        self.pc = 512;

        Ok(())
    }

    pub fn load_font(&mut self) {
        for (idx, val) in FONT.iter().enumerate() {
            self.memory[idx] = *val;
        }
    }

    fn increase_pc(&mut self) {
        self.pc += 2
    }

    pub fn fetch(&mut self) {
        let part_a = self.memory[self.pc] as u16;
        let part_b = self.memory[self.pc + 1] as u16;

        self.instruction = (part_a << 8) | part_b;
        self.increase_pc();
    }

    pub fn execute(&mut self) {
        let opcode = self.instruction;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        /* 00E0: clear screen */
                        self.clear_screen()
                    }
                    0x00EE => { /* 00EE: return */ }
                    other => todo!("unimplemented opcode: {}", other),
                }
            }

            0x1000 => {
                /* 1NNN: jump to address */
                let addr = opcode & 0x0FFF;

                self.pc = addr.into();
            }
            0x2000 => { /* 2NNN: call subroutine */ }
            0x3000 => {
                /* 3XNN: skip if Vx == NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register as usize] == value {
                    self.increase_pc();
                }
            }
            0x4000 => {
                /* 4XNN: skip if Vx != NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register as usize] != value {
                    self.increase_pc();
                }
            }
            0x5000 => {
                /* 5XY0: skip if Vx == Vy */
                let left = (opcode & 0x0F00) >> 8;
                let right = (opcode & 0x00F0) >> 4;

                if self.registers[left as usize] == self.registers[right as usize] {
                    self.increase_pc();
                }
            }
            0x6000 => {
                /* 6XNN: Vx = Vy */
                let left = (opcode & 0x0F00) >> 8;
                let right = (opcode & 0x00F0) >> 4;

                self.registers[left as usize] = self.registers[right as usize];
            }
            0x7000 => {
                /* 7XNN: Vx += NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                self.registers[register as usize] += value;
            }

            0x8000 => { /* 8XY0, 8XY1, 8XY2, 8XY3, 8XY4, 8XY5, 8XY6, 8XY7, 8XYE  */ }

            0x9000 => {
                /* 9XY0: if(Vx != Vy) */
                let left = (opcode & 0x0F00) >> 8;
                let right = (opcode & 0x00F0) >> 4;

                if self.registers[left as usize] != self.registers[right as usize] {
                    self.increase_pc();
                }
            }
            0xA000 => {
                /* ANNN: I = NNN */
                let value = opcode & 0x0FFF;

                self.index_register = value;
            }
            0xB000 => {
                /* BNNN: PC = V0 + NNN */
                let value = opcode & 0x0FFF;

                self.pc = (self.registers[0] as u16 + value) as usize;
            }
            0xC000 => {
                /* CXNN: Vx = rand() & NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                let rand = rand::random::<u8>();

                self.registers[register as usize] = rand & value;
            }
            0xD000 => {
                /* DXYN: draw(Vx, Vy, N) */
                let register_a = (opcode & 0x0F00) >> 8;
                let register_b = (opcode & 0x00F0) >> 4;
                let height = opcode & 0x000F;

                let coord_a = self.registers[register_a as usize];
                let coord_b = self.registers[register_b as usize];
            }
            0xE000 => {
                /* EX9E, EXA1 */
                let register = (opcode & 0x0F00) >> 8;

                match opcode & 0x00FF {
                    0x009E => { /* EX9E: */ }
                    0x00A1 => { /* EXA1: */ }
                    other => todo!("unimplemented opcode: {}", other),
                };
            }
            0xF000 => {
                /* FX07, FX0A, FX15, FX18, FX29, FX33, FX55, FX65 */
                let register = (opcode & 0x0F00) >> 8;

                match opcode & 0x00FF {
                    0x0007 => {}
                    0x000A => {}
                    0x0015 => {}
                    0x0018 => {}
                    0x0029 => {}
                    0x0033 => {}
                    0x0055 => {}
                    0x0065 => {}
                    other => todo!("unimplemented opcode: {}", other),
                }
            }

            other => todo!("unimplemented opcode: {}", other),
        }
    }

    fn clear_screen(&mut self) {
        self.buffer = vec![0u32; WIDTH * HEIGHT];
    }

    #[inline(always)]
    fn draw(&mut self, (x, y): (usize, usize)) {
        for i in 0..12 {
            for j in 0..12 {
                self.draw_raw((x + i, y + j))
            }
        }
    }

    #[inline(always)]
    fn draw_raw(&mut self, (x, y): (usize, usize)) {
        let pos = WIDTH * y + x;

        self.buffer[pos] = 0xFFFFFF;
    }
}
