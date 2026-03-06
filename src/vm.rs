use std::io::Read;

use anyhow::bail;
use log::info;

use crate::{GRID_HEIGHT, GRID_WIDTH, HEIGHT, SCALE, WIDTH};

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

macro_rules! draw_bit {
    ($bit:ident, $x:ident, $y:ident, $self:ident) => {
        if $bit == 1 {
            if $self.pixel_at(($x, $y)) {
                $self.set_pixel(($x, $y), false);
                $self.registers[0xF] = 1;
            } else {
                $self.set_pixel(($x, $y), true);
            }
        }
        $x += 1;

        if $x >= GRID_WIDTH {
            break;
        }
    };
}

#[derive(Debug)]
pub struct VM {
    memory: [u8; 4096],

    registers: [u8; 16],
    address_regiser: u16,
    index_register: u16,
    pc: usize,
    instruction: u16,

    stack: Vec<u16>,

    pixels: Vec<bool>,
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
            pixels: vec![false; GRID_WIDTH * GRID_HEIGHT],
        }
    }

    pub fn render(&mut self) -> Vec<u32> {
        let mut buffer = vec![0x00; WIDTH * HEIGHT];

        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let val = self.pixel_at((x, y));

                for xi in 0..SCALE {
                    for yi in 0..SCALE {
                        buffer[WIDTH * (y * SCALE + yi) + (x * SCALE + xi)] = match val {
                            true => 0xFFFF,
                            false => 0x0000,
                        };
                    }
                }
            }
        }

        buffer
    }

    pub fn load(&mut self, mut reader: impl Read) -> anyhow::Result<()> {
        let mut buf: [u8; GRID_WIDTH] = [0x00; GRID_WIDTH];

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
                        info!("00E0");
                        self.clear_screen()
                    }
                    0x00EE => {
                        /* 00EE: return */
                        info!("00EE");
                    }
                    other => todo!("unimplemented opcode: {}", other),
                }
            }

            0x1000 => {
                /* 1NNN: jump to address */
                info!("1NNN");
                let addr = opcode & 0x0FFF;

                self.pc = addr.into();
            }
            0x2000 => { /* 2NNN: call subroutine */ }
            0x3000 => {
                info!("3XNN");
                /* 3XNN: skip if Vx == NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register as usize] == value {
                    self.increase_pc();
                }
            }
            0x4000 => {
                info!("4XNN");
                /* 4XNN: skip if Vx != NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                if self.registers[register as usize] != value {
                    self.increase_pc();
                }
            }
            0x5000 => {
                info!("5XY0");
                /* 5XY0: skip if Vx == Vy */
                let left = (opcode & 0x0F00) >> 8;
                let right = (opcode & 0x00F0) >> 4;

                if self.registers[left as usize] == self.registers[right as usize] {
                    self.increase_pc();
                }
            }
            0x6000 => {
                info!("6XNN");
                /* 6XNN: Vx = Vy */
                let left = (opcode & 0x0F00) >> 8;
                let right = (opcode & 0x00F0) >> 4;

                self.registers[left as usize] = self.registers[right as usize];
            }
            0x7000 => {
                info!("7XNN");
                /* 7XNN: Vx += NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                self.registers[register as usize] += value;
            }

            0x8000 => {
                /* 8XY0, 8XY1, 8XY2, 8XY3, 8XY4, 8XY5, 8XY6, 8XY7, 8XYE  */
                info!("8???");
            }

            0x9000 => {
                info!("9XY0");
                /* 9XY0: if(Vx != Vy) */
                let left = (opcode & 0x0F00) >> 8;
                let right = (opcode & 0x00F0) >> 4;

                if self.registers[left as usize] != self.registers[right as usize] {
                    self.increase_pc();
                }
            }
            0xA000 => {
                info!("ANNN");
                /* ANNN: I = NNN */
                let value = opcode & 0x0FFF;

                self.index_register = value;
            }
            0xB000 => {
                info!("B000");
                /* BNNN: PC = V0 + NNN */
                let value = opcode & 0x0FFF;

                self.pc = (self.registers[0] as u16 + value) as usize;
            }
            0xC000 => {
                info!("C000");
                /* CXNN: Vx = rand() & NN */
                let register = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;

                let rand = rand::random::<u8>();

                self.registers[register as usize] = rand & value;
            }
            0xD000 => {
                info!("D000");
                /* DXYN: draw(Vx, Vy, N) */
                let register_x = (opcode & 0x0F00) >> 8;
                let register_y = (opcode & 0x00F0) >> 4;

                let height = opcode & 0x000F;

                let mut x = self.registers[register_x as usize] as usize % WIDTH;
                let mut y = self.registers[register_y as usize] as usize % HEIGHT;

                self.registers[0xF] = 0;

                let index = self.index_register;

                for i in 0..height {
                    let data = self.memory[(index + i) as usize];

                    let bit_1 = (data & 0b10000000) >> 7;
                    let bit_2 = (data & 0b01000000) >> 6;
                    let bit_3 = (data & 0b00100000) >> 5;
                    let bit_4 = (data & 0b00010000) >> 4;
                    let bit_5 = (data & 0b00001000) >> 3;
                    let bit_6 = (data & 0b00000100) >> 2;
                    let bit_7 = (data & 0b00000010) >> 1;
                    let bit_8 = data & 0b00000001;

                    draw_bit!(bit_1, x, y, self);
                    draw_bit!(bit_2, x, y, self);
                    draw_bit!(bit_3, x, y, self);
                    draw_bit!(bit_4, x, y, self);
                    draw_bit!(bit_5, x, y, self);
                    draw_bit!(bit_6, x, y, self);
                    draw_bit!(bit_7, x, y, self);
                    draw_bit!(bit_8, x, y, self);

                    y += 1;
                    x = self.registers[register_x as usize] as usize % WIDTH;

                    if y >= GRID_HEIGHT {
                        break;
                    }
                }
            }
            0xE000 => {
                info!("E000");
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
                    0x0007 => info!("FX07"),
                    0x000A => info!("FX0A"),
                    0x0015 => info!("FX15"),
                    0x0018 => info!("FX18"),
                    0x0029 => info!("FX29"),
                    0x0033 => info!("FX33"),
                    0x0055 => info!("FX55"),
                    0x0065 => info!("FX65"),
                    other => todo!("unimplemented opcode: {}", other),
                }
            }

            other => todo!("unimplemented opcode: {}", other),
        }
    }

    fn clear_screen(&mut self) {
        self.pixels = vec![false; GRID_WIDTH * GRID_HEIGHT];
    }

    fn pixel_at(&self, (x, y): (usize, usize)) -> bool {
        self.pixels[GRID_WIDTH * y + x]
    }

    fn set_pixel(&mut self, (x, y): (usize, usize), val: bool) {
        self.pixels[GRID_WIDTH * y + x] = val
    }

    // #[inline(always)]
    // fn draw(&mut self, (x, y): (usize, usize)) {
    //     for i in 0..SCALE {
    //         for j in 0..SCALE {
    //             self.draw_raw((x + i, y + j))
    //         }
    //     }
    // }
    //
    // #[inline(always)]
    // fn draw_raw(&mut self, (x, y): (usize, usize)) {
    //     let pos = WIDTH * y + x;
    //
    //     self.buffer[pos] = 0xFFFFFF;
    // }
}
