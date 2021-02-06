#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Giraffics")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            let colors = [rgb(255, 0, 0), rgb(0, 255, 0), rgb(0, 0, 255)];

            for x in WIDTH / 2 - 20..WIDTH / 2 + 20 {
                for y in HEIGHT / 2 - 20..HEIGHT / 2 + 20 {
                    put_pixel(frame, x, y, &colors[(x + y) % 3]);
                }
            }

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

#[repr(transparent)]
struct Color([u8; 4]);

#[inline]
fn rgb(red: u8, green: u8, blue: u8) -> Color {
    Color([red, green, blue, 255])
}

#[inline]
fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color([red, green, blue, alpha])
}

#[inline]
fn put_pixel(frame: &mut [u8], x: usize, y: usize, color: &Color) {
    let pixel_index = (y * WIDTH + x) * 4;
    let pixel = &mut frame[pixel_index..pixel_index + 4];
    pixel.copy_from_slice(&color.0)
}
