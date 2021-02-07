use std::ops::{Add, Mul};

pub(crate) const RED: Color = Color::rgb(255, 0, 0);
pub(crate) const GREEN: Color = Color::rgb(0, 255, 0);
pub(crate) const BLUE: Color = Color::rgb(0, 0, 255);
pub(crate) const BLACK: Color = Color::rgb(0, 0, 0);
pub(crate) const WHITE: Color = Color::rgb(255, 255, 255);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct Color([u8; 4]);

impl Color {
    pub(crate) fn as_array<'a>(&'a self) -> &'a [u8; 4] {
        &self.0
    }

    #[inline]
    pub(crate) const fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color([red, green, blue, 255])
    }

    #[inline]
    pub(crate) const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color([red, green, blue, alpha])
    }

    pub(crate) fn red(&self) -> u8 {
        self.0[0]
    }

    pub(crate) fn green(&self) -> u8 {
        self.0[1]
    }

    pub(crate) fn blue(&self) -> u8 {
        self.0[2]
    }

    pub(crate) fn alpha(&self) -> u8 {
        self.0[3]
    }

    pub(crate) fn scale(&self, scalar: f64) -> Self {
        let red = mul_with_ceiling(self.red(), scalar);
        let green = mul_with_ceiling(self.green(), scalar);
        let blue = mul_with_ceiling(self.blue(), scalar);
        let alpha = mul_with_ceiling(self.alpha(), scalar);

        Color::rgba(red, green, blue, alpha)
    }

    pub(crate) fn from_rgb_tuple(tuple: (f64, f64, f64)) -> Self {
        let (red, green, blue) = tuple;

        Self::rgb(red as u8, green as u8, blue as u8)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let red = add_with_ceiling(self.red(), other.red());
        let green = add_with_ceiling(self.green(), other.green());
        let blue = add_with_ceiling(self.blue(), other.blue());
        let alpha = add_with_ceiling(self.alpha(), other.alpha());

        Color::rgba(red, green, blue, alpha)
    }
}

impl<T> Mul<T> for Color
where
    T: Into<f64>,
{
    type Output = Color;

    fn mul(self, other: T) -> Color {
        self.scale(other.into())
    }
}

fn add_with_ceiling(a: u8, b: u8) -> u8 {
    let sum = (a as u16) + (b as u16);
    if sum > (u8::MAX as u16) {
        u8::MAX
    } else {
        sum as u8
    }
}

fn mul_with_ceiling(a: u8, b: f64) -> u8 {
    let sum = (a as f64) * b;
    if sum > (u8::MAX as f64) {
        u8::MAX
    } else {
        sum as u8
    }
}
