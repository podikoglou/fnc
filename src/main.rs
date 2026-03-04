use std::{fs::File, io::BufReader, os, path::PathBuf, sync::Arc};

use crate::vm::VM;
use anyhow::Context;
use minifb::{Key, ScaleMode, Window, WindowOptions};

mod vm;

const WIDTH: usize = 64 * 12;
const HEIGHT: usize = 32 * 12;

fn main() -> anyhow::Result<()> {
    // read file
    let mut args = std::env::args().skip(1);

    let path = args.next().map(PathBuf::from).context("invalid path")?;
    let file = File::open(path).context("couldn't open file")?;
    let reader = BufReader::new(file);

    // create vm
    let mut vm = VM::new();
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

    window.set_target_fps(30);

    // render loop
    // let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // handle resizing
        let new_size = window.get_size();
        // if new_size != size {
        //     size = new_size;
        //     buffer.resize(size.0 * size.1, 0);
        // }
        //
        // render
        let buffer = vm.render();

        window.update_with_buffer(buffer, new_size.0, new_size.1)?;
    }

    Ok(())
}
