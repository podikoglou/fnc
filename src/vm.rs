use std::io::Read;

#[derive(Debug)]
pub struct VM {
    memory: [u8; 4096],

    registers: [u8; 16],
    address_regiser: u16,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0x00; 4096],
            registers: [0x00; 16],
            address_regiser: 0x00,
            pc: 0,
        }
    }

    pub fn render(&mut self, frame: &mut [u8]) {}

    pub fn load(&self, reader: impl Read) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn execute_instruction(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => { /* clear screen */ }
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
}
