use std::{fs::File, io::BufReader, os, path::PathBuf, sync::Arc};

use anyhow::Context;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

use crate::vm::VM;

mod vm;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().map(PathBuf::from).context("invalid path")?;
    let file = File::open(path).context("couldn't open file")?;
    let reader = BufReader::new(file);

    // create vm
    let mut vm = VM::new();
    vm.load(reader)?;

    // create window
    let event_loop = EventLoop::new().unwrap();

    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        #[allow(deprecated)]
        Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("FnC")
                        .with_inner_size(size)
                        .with_min_inner_size(size),
                )
                .unwrap(),
        )
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    #[allow(deprecated)]
    let res = event_loop.run(|event, elwt| {
        match event {
            Event::Resumed => {}
            Event::NewEvents(_) => input.step(),
            Event::AboutToWait => input.end_step(),
            Event::DeviceEvent { event, .. } => {
                input.process_device_event(&event);
            }
            Event::WindowEvent { event, .. } => {
                // Draw the current frame
                if event == WindowEvent::RedrawRequested {
                    // world.draw(pixels.frame_mut());
                    vm.render(pixels.frame_mut());

                    if let Err(err) = pixels.render() {
                        dbg!(err);
                        elwt.exit();
                        return;
                    }
                }

                // Handle input events
                if input.process_window_event(&event) {
                    // Close events
                    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                        elwt.exit();
                        return;
                    }

                    // Resize the window
                    if let Some(size) = input.window_resized() {
                        if let Err(err) = pixels.resize_surface(size.width, size.height) {
                            dbg!(err);
                            elwt.exit();
                            return;
                        }
                    }

                    // Update internal state and request a redraw
                    // world.update();
                    window.request_redraw();
                }
            }
            _ => {}
        }
    });

    Ok(())
}
