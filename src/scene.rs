use crate::canvas::{Canvas, CanvasCoordinate};
use crate::color::Color;
use crate::shape::sphere::Sphere;
use crate::traits::Converts;
use std::ops::Sub;

pub(crate) const ORIGIN: WorldCoordinate = WorldCoordinate {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct WorldCoordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl WorldCoordinate {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Sub<WorldCoordinate> for WorldCoordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        WorldCoordinate::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

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
}

impl Scene {
    pub(crate) fn new(
        camera_position: WorldCoordinate,
        viewport: ViewPort,
        canvas: Canvas,
        background_color: Color,
    ) -> Self {
        Scene {
            camera_position,
            viewport,
            canvas,
            spheres: vec![],
            background_color,
        }
    }

    pub(crate) fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere)
    }

    pub(crate) fn trace_ray(
        &self,
        viewport_coord: WorldCoordinate,
        t_min: f64,
        t_max: f64,
    ) -> Color {
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
            Some(s) => s.color(),
            None => self.background_color,
        }
    }

    pub(crate) fn render(&self, frame: &mut [u8]) {
        for coord in self.canvas.iter_pixels() {
            let viewport_coord = self.convert(coord);
            let color = self.trace_ray(viewport_coord, 1f64, f64::INFINITY);
            self.canvas.put_pixel(frame, coord, color);
        }
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
