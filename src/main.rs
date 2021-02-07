mod canvas;
mod color;
mod scene;
mod shape;
mod traits;

use canvas::Canvas;
use color::{BLACK, BLUE, GREEN, RED};
use log::error;
use pixels::{Error, SurfaceTexture};
use scene::{Scene, ViewPort, WorldCoordinate, ORIGIN};
use shape::sphere::Sphere;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

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
    let mut scene = Scene::new(ORIGIN, ViewPort::default(), canvas, BLACK);
    scene.add_sphere(Sphere::new(1.0, WorldCoordinate::new(0.0, -1.0, 3.0), RED));
    scene.add_sphere(Sphere::new(1.0, WorldCoordinate::new(2.0, 0.0, 4.0), BLUE));
    scene.add_sphere(Sphere::new(
        1.0,
        WorldCoordinate::new(-2.0, 0.0, 4.0),
        GREEN,
    ));

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            scene.render(frame);

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
