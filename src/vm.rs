use std::io::Read;

#[derive(Debug)]
pub struct VM {}

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, frame: &mut [u8]) {}

    pub fn load(&self, reader: impl Read) -> anyhow::Result<()> {
        Ok(())
    }
}
