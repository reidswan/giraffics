pub(crate) mod object;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::coord::{CanvasCoordinate, WorldCoordinate};
use crate::traits::Converts;
use object::light::Light;
use object::shape::Sphere;

#[derive(Copy, Clone)]
pub(crate) struct ViewPort {
    depth: usize,
    width: usize,
    height: usize,
}

impl Default for ViewPort {
    fn default() -> Self {
        Self {
            depth: 1,
            width: 1,
            height: 1,
        }
    }
}

pub(crate) struct Scene {
    camera_position: WorldCoordinate,
    viewport: ViewPort,
    canvas: Canvas,
    spheres: Vec<Sphere>,
    background_color: Color,
    lights: Vec<Light>,
    title: String,
}

impl Scene {
    pub(crate) fn new(
        camera_position: WorldCoordinate,
        viewport: ViewPort,
        canvas: Canvas,
        background_color: Color,
        title: String,
    ) -> Self {
        Scene {
            camera_position,
            viewport,
            canvas,
            spheres: vec![],
            background_color,
            lights: vec![],
            title,
        }
    }

    pub(crate) fn with_lights(mut self, lights: Vec<Light>) -> Self {
        self.lights = lights;
        self
    }

    pub(crate) fn with_spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = spheres;
        self
    }

    fn trace_ray(&self, viewport_coord: WorldCoordinate, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = f64::INFINITY;
        let mut closest_sphere: Option<&Sphere> = None;
        for sphere in self.spheres.iter() {
            let (t1, t2) = sphere.intersect_ray(self.camera_position, viewport_coord);
            if t_min <= t1 && t1 <= t_max && t1 < closest_t {
                closest_sphere = Some(sphere);
                closest_t = t1
            }
            if t_min <= t2 && t2 <= t_max && t2 < closest_t {
                closest_sphere = Some(sphere);
                closest_t = t2
            }
        }

        match closest_sphere {
            Some(s) => {
                let color = s.color();
                let point = self.camera_position + viewport_coord * closest_t;
                let normal = {
                    let normal_dir = point - s.center();
                    normal_dir / normal_dir.abs()
                };
                let light_intensity = self.compute_lighting(point, normal);

                color.scale(light_intensity)
            }
            None => self.background_color,
        }
    }

    fn compute_lighting(&self, point: WorldCoordinate, normal: WorldCoordinate) -> f64 {
        self.lights
            .iter()
            .map(|l| l.illumination_at_point(point, normal))
            .sum()
    }

    pub(crate) fn render(&self, frame: &mut [u8]) {
        for coord in self.canvas.iter_pixels() {
            let viewport_coord = self.convert(coord);
            let color = self.trace_ray(viewport_coord, 1f64, f64::INFINITY);
            self.canvas.put_pixel(frame, coord, color);
        }
    }

    pub(crate) fn canvas(&self) -> Canvas {
        self.canvas
    }
}

impl Converts<CanvasCoordinate, WorldCoordinate> for Scene {
    fn convert(&self, coord: CanvasCoordinate) -> WorldCoordinate {
        let x = coord.x as f64 * (self.viewport.width as f64 / self.canvas.width() as f64);
        let y = coord.y as f64 * (self.viewport.height as f64 / self.canvas.height() as f64);
        let z = self.viewport.depth as f64;
        WorldCoordinate::new(x, y, z)
    }
}
