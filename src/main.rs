mod canvas;
mod color;
mod coord;
mod lang;
mod scene;
mod traits;

use canvas::Canvas;
use color::{Color, BLACK};
use coord::{WorldCoordinate, ORIGIN};
use lang::parser::{LightDefinition, Parser, SceneDefinition};
use log::error;
use pixels::SurfaceTexture;
use scene::object::light::Light;
use scene::object::shape::Sphere;
use scene::{Scene, ViewPort};
use std::env;
use std::fs;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), String> {
    env_logger::init();
    let args: Vec<_> = env::args().collect();
    let file = if args.len() >= 2 { &args[1] } else { "" };
    if file.is_empty() {
        return Err("Supply a valid file name".into());
    }

    let contents =
        fs::read_to_string(&file).map_err(|e| format!("Failed to read '{}': {}", &file, e))?;
    let mut parser = Parser::new(&contents);
    let definitions = parser.parse()?;

    let scene = load_scene(definitions);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let canvas = scene.canvas();
    let window = canvas.window().build(&event_loop).unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        canvas
            .create_pixels(surface_texture)
            .map_err(|e| format!("Failed to create canvas {:?}", e))?
    };

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

fn load_scene(definition: SceneDefinition) -> Scene {
    let mut window_width = canvas::DEFAULT_WIDTH;
    let mut window_height = canvas::DEFAULT_HEIGHT;
    let mut canvas = Canvas::default();
    let mut window_title = String::from("Giraffics");
    if let Some(window_def) = definition.window {
        if let Some(width) = window_def.width {
            window_width = width as usize;
        }
        if let Some(height) = window_def.height {
            window_height = height as usize;
        }
        if let Some(title) = window_def.title {
            window_title = title;
        }
    }

    let lights = definition
        .lights
        .into_iter()
        .map(|light_def| match light_def {
            LightDefinition::AmbientLight { intensity } => Light::ambient(intensity),
            LightDefinition::DirectionLight {
                intensity,
                direction,
            } => Light::direction(WorldCoordinate::from_tuple(direction), intensity),
            LightDefinition::PointLight {
                intensity,
                position,
            } => Light::point(WorldCoordinate::from_tuple(position), intensity),
        })
        .collect();

    let spheres = definition
        .spheres
        .into_iter()
        .map(|sphere| {
            Sphere::new(
                sphere.radius,
                WorldCoordinate::from_tuple(sphere.center),
                Color::from_rgb_tuple(sphere.color),
            )
        })
        .collect();

    canvas = canvas.with_height(window_height).with_width(window_width);
    Scene::new(ORIGIN, ViewPort::default(), canvas, BLACK, window_title)
        .with_lights(lights)
        .with_spheres(spheres)
}
