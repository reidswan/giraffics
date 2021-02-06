#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::random;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

const DEFAULT_WIDTH: usize = 320;
const DEFAULT_HEIGHT: usize = 240;

#[derive(Copy, Clone)]
struct Canvas {
    width: usize,
    height: usize,
}

impl Canvas {
    fn logical_size(&self) -> LogicalSize<f64> {
        LogicalSize::new(self.width as f64, self.height as f64)
    }

    fn create_pixels(
        &self,
        surface_texture: SurfaceTexture<'_, Window>,
    ) -> Result<Pixels<Window>, Error> {
        Pixels::new(self.width as u32, self.height as u32, surface_texture)
    }

    #[inline]
    fn put_pixel<T>(&self, frame: &mut [u8], coord: T, color: &Color)
    where
        Self: Converts<T, ScreenCoordinate>,
    {
        let screen_coord = self.convert(coord);
        if let ScreenCoordinate::OnScreen { x, y } = screen_coord {
            let pixel_index = (y * self.width + x) * 4;
            let pixel = &mut frame[pixel_index..pixel_index + 4];
            pixel.copy_from_slice(&color.0)
        }
        // otherwise do nothing
    }

    fn window(&self) -> WindowBuilder {
        let size = self.logical_size();
        WindowBuilder::new()
            .with_title("Giraffics")
            .with_inner_size(size)
            .with_min_inner_size(size)
    }
}

trait Converts<A, B> {
    fn convert(&self, a: A) -> B;
}

impl Converts<CanvasCoordinate, ScreenCoordinate> for Canvas {
    fn convert(&self, coord: CanvasCoordinate) -> ScreenCoordinate {
        let x = (self.width / 2) as isize + coord.x;
        let y = (self.height / 2) as isize + coord.y;
        if out_of_bounds(x, y, self.width as isize, self.height as isize) {
            ScreenCoordinate::OffScreen
        } else {
            ScreenCoordinate::OnScreen {
                x: x as usize,
                y: y as usize,
            }
        }
    }
}

impl Converts<ScreenCoordinate, ScreenCoordinate> for Canvas {
    fn convert(&self, coord: ScreenCoordinate) -> ScreenCoordinate {
        coord
    }
}

fn out_of_bounds(x: isize, y: isize, width: isize, height: isize) -> bool {
    x < 0 || y < 0 || x > width || y > height
}

/**
 * A coordinate system starting at (0, 0) in the top left and increasing monotically in both axes to the right and down
 */
#[derive(Copy, Clone)]
enum ScreenCoordinate {
    OffScreen,
    OnScreen { x: usize, y: usize },
}

/**
 * A coordinate system centered on (0, 0), with corners at
 * (-width/2, -height/2), (width/2, -height/2), (-width/2, height/2), (width/2, height/2)
 */
#[derive(Copy, Clone)]
struct CanvasCoordinate {
    x: isize,
    y: isize,
}

impl CanvasCoordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let canvas = Canvas::default();
    let window = canvas.window().build(&event_loop).unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        canvas.create_pixels(surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        let width = canvas.width;
        let height = canvas.height;
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            let colors = [rgb(255, 0, 0), rgb(0, 255, 0), rgb(0, 0, 255)];
            let rect_width = (width / 10) as isize;
            let rect_height = (height / 10) as isize;
            for x in -rect_width..rect_width {
                for y in -rect_height..rect_height {
                    let color_index = random::<usize>() % 3;
                    let coord = CanvasCoordinate::new(x, y);
                    canvas.put_pixel(frame, coord, &colors[color_index]);
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
