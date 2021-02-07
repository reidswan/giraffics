use crate::color::Color;
use crate::scene::WorldCoordinate;

#[derive(Copy, Clone)]
pub(crate) struct Sphere {
    radius: f64,
    center: WorldCoordinate,
    color: Color,
}

impl Sphere {
    pub(crate) fn intersect_ray(
        &self,
        camera: WorldCoordinate,
        viewport: WorldCoordinate,
    ) -> (f64, f64) {
        let r = self.radius;
        let vec_co = camera - self.center;

        let a = viewport.dot(viewport);
        let b = 2.0 * vec_co.dot(viewport);
        let c = vec_co.dot(vec_co) - r * r;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            (f64::INFINITY, f64::INFINITY)
        } else {
            let t1 = (-b + disc.sqrt()) / (2.0 * a);
            let t2 = (-b - disc.sqrt()) / (2.0 * a);

            (t1, t2)
        }
    }

    pub(crate) fn color(self) -> Color {
        self.color
    }

    pub(crate) fn center(self) -> WorldCoordinate {
        self.center
    }

    pub(crate) fn new(radius: f64, center: WorldCoordinate, color: Color) -> Self {
        Self {
            radius,
            center,
            color,
        }
    }
}
