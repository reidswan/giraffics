use std::ops::{Add, Div, Mul, Sub};

pub(crate) const ORIGIN: WorldCoordinate = WorldCoordinate {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

/**
 * A coordinate for a position (or direction) within a 3D world
 */
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

    pub(crate) fn abs(self) -> f64 {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub(crate) fn from_tuple(tuple: (f64, f64, f64)) -> Self {
        let (x, y, z) = tuple;
        Self { x, y, z }
    }
}

impl Sub<WorldCoordinate> for WorldCoordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        WorldCoordinate::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for WorldCoordinate {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let Self { x, y, z } = self;

        WorldCoordinate::new(x * other, y * other, z * other)
    }
}

impl Add<WorldCoordinate> for WorldCoordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Self { x, y, z } = self;
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = other;

        WorldCoordinate::new(x + x1, y + y1, z + z1)
    }
}

impl Div<f64> for WorldCoordinate {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        let Self { x, y, z } = self;
        WorldCoordinate::new(x / other, y / other, z / other)
    }
}

/**
 * A coordinate system starting at (0, 0) in the top left and increasing monotically in both axes to the right and down
 */
#[derive(Copy, Clone)]
pub(crate) enum ScreenCoordinate {
    OffScreen,
    OnScreen { x: usize, y: usize },
}

/**
 * A coordinate system centered on (0, 0), with corners at
 * (-width/2, -height/2), (width/2, -height/2), (-width/2, height/2), (width/2, height/2)
 */
#[derive(Copy, Clone, Debug)]
pub(crate) struct CanvasCoordinate {
    pub x: isize,
    pub y: isize,
}

impl CanvasCoordinate {
    pub(crate) fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
