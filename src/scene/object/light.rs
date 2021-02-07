use crate::scene::WorldCoordinate;

#[derive(Copy, Clone)]
pub(crate) enum Light {
    Point {
        position: WorldCoordinate,
        intensity: f64,
    },
    Direction {
        direction: WorldCoordinate,
        intensity: f64,
    },
    Ambient {
        intensity: f64,
    },
}

impl Light {
    pub(crate) fn illumination_at_point(
        self,
        point: WorldCoordinate,
        surface_normal: WorldCoordinate,
    ) -> f64 {
        match self {
            Self::Ambient { intensity } => intensity,
            Self::Direction {
                direction,
                intensity,
            } => directional_intensity(direction, surface_normal, intensity),
            Self::Point {
                position,
                intensity,
            } => directional_intensity(position - point, surface_normal, intensity),
        }
    }

    pub(crate) fn ambient(intensity: f64) -> Self {
        Self::Ambient { intensity }
    }

    pub(crate) fn point(position: WorldCoordinate, intensity: f64) -> Self {
        Self::Point {
            intensity,
            position,
        }
    }

    pub(crate) fn direction(direction: WorldCoordinate, intensity: f64) -> Self {
        Self::Direction {
            intensity,
            direction,
        }
    }
}

fn directional_intensity(
    direction: WorldCoordinate,
    surface_normal: WorldCoordinate,
    intensity: f64,
) -> f64 {
    let n_dot_l = surface_normal.dot(direction);
    if n_dot_l > 0.0 {
        intensity * n_dot_l / (surface_normal.abs() * direction.abs())
    } else {
        0.0
    }
}
