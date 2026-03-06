use std::{fs::File, io::BufReader, path::PathBuf};

use crate::vm::VM;
use anyhow::Context;
use minifb::{Key, ScaleMode, Window, WindowOptions};

mod vm;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 32;

const WIDTH: usize = GRID_WIDTH * 12;
const HEIGHT: usize = GRID_HEIGHT * 12;

const SCALE: usize = 12;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // read file
    let mut args = std::env::args().skip(1);

    let path = args.next().map(PathBuf::from).context("invalid path")?;
    let file = File::open(path).context("couldn't open file")?;
    let reader = BufReader::new(file);

    // create vm
    let mut vm = VM::new();
    vm.load_font();
    vm.load(reader)?;

    // create window
    let mut window = Window::new(
        "FnC",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )?;

    window.set_target_fps(60);

    // render loop
    // let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        vm.fetch();
        vm.execute();

        // handle resizing
        let new_size = window.get_size();
        // if new_size != size {
        //     size = new_size;
        //     buffer.resize(size.0 * size.1, 0);
        // }
        //
        // render
        let buffer = vm.render();

        window.update_with_buffer(&buffer, new_size.0, new_size.1)?;
    }

    Ok(())
}
