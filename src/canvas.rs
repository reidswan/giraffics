use crate::color::Color;
use crate::traits::Converts;
use pixels::{Error, Pixels, SurfaceTexture};
use std::iter::Iterator;
use winit::dpi::LogicalSize;
use winit::window::{Window, WindowBuilder};

const DEFAULT_WIDTH: usize = 1000;
const DEFAULT_HEIGHT: usize = 700;

#[derive(Copy, Clone)]
pub(crate) struct Canvas {
    width: usize,
    height: usize,
}

impl Canvas {
    fn logical_size(self) -> LogicalSize<f64> {
        LogicalSize::new(self.width as f64, self.height as f64)
    }

    pub(crate) fn create_pixels(
        self,
        surface_texture: SurfaceTexture<'_, Window>,
    ) -> Result<Pixels<Window>, Error> {
        Pixels::new(self.width as u32, self.height as u32, surface_texture)
    }

    #[inline]
    pub(crate) fn put_pixel<T>(self, frame: &mut [u8], coord: T, color: Color)
    where
        Self: Converts<T, ScreenCoordinate>,
    {
        let screen_coord = self.convert(coord);
        if let ScreenCoordinate::OnScreen { x, y } = screen_coord {
            let pixel_index = (y * self.width + x) * 4;
            let pixel = &mut frame[pixel_index..pixel_index + 4];
            pixel.copy_from_slice(color.as_array())
        }
        // otherwise do nothing
    }

    pub(crate) fn window(self) -> WindowBuilder {
        let size = self.logical_size();
        WindowBuilder::new()
            .with_title("Giraffics")
            .with_inner_size(size)
            .with_min_inner_size(size)
    }
    pub(crate) fn width(self) -> usize {
        self.width
    }
    pub(crate) fn height(self) -> usize {
        self.width
    }

    pub(crate) fn iter_pixels(self) -> EachCanvasCoordinate {
        let max_x = (self.width as isize) / 2;
        let min_x = -max_x;
        let max_y = (self.height as isize) / 2;
        let min_y = -max_y;
        let finished = false;
        let next_x = min_x;
        let next_y = min_y;

        EachCanvasCoordinate {
            max_x,
            min_x,
            max_y,
            finished,
            next_x,
            next_y,
        }
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

impl Converts<CanvasCoordinate, ScreenCoordinate> for Canvas {
    fn convert(&self, coord: CanvasCoordinate) -> ScreenCoordinate {
        let x = (self.width / 2) as isize + coord.x;
        let y = (self.height / 2) as isize - coord.y;
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
    x < 0 || y < 0 || x >= width || y >= height
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

pub(crate) struct EachCanvasCoordinate {
    min_x: isize,
    max_x: isize,
    max_y: isize,
    next_x: isize,
    next_y: isize,
    finished: bool,
}

impl Iterator for EachCanvasCoordinate {
    type Item = CanvasCoordinate;

    fn next(&mut self) -> Option<CanvasCoordinate> {
        if self.finished {
            return None;
        }

        let coord = Some(CanvasCoordinate::new(self.next_x, self.next_y));

        self.next_x += 1;
        if self.next_x >= self.max_x as isize {
            self.next_x = self.min_x;
            self.next_y += 1;
            if self.next_y >= self.max_y {
                self.finished = true
            }
        }

        coord
    }
}

impl IntoIterator for Canvas {
    type Item = CanvasCoordinate;
    type IntoIter = EachCanvasCoordinate;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_pixels()
    }
}
