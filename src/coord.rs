use std::ops::*;
type Point = ggez::graphics::mint::Point2<f32>;
use winit::dpi::{Position, PhysicalPosition};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Into<Coord>> Add<T> for Coord {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        Self::new(
            self.x + other.x,
            self.y + other.y,
            )
    }
}

impl<T: Into<Coord>> AddAssign<T> for Coord {
    fn add_assign(&mut self, other: T) {
        *self = *self + other
    }
}

impl<T: Into<Coord>> Sub<T> for Coord {
    type Output = Self;
    fn sub(self, other: T) -> Self::Output {
        let other = other.into();
        Self::new(
            self.x - other.x,
            self.y - other.y,
            )
    }
}

impl<T: Into<Coord>> SubAssign<T> for Coord {
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

impl Mul<f32> for Coord {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self::new(self.x * other, self.y * other)
    }
}

impl MulAssign<f32> for Coord {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl From<(f32, f32)> for Coord {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<[f32; 2]> for Coord {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Coord> for Point {
    fn from(Coord { x, y }: Coord) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<Coord> for Position {
    fn from(Coord { x, y }: Coord) -> Self {
        Position::Physical(PhysicalPosition::new(x as i32, y as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Coord::new(12., 15.), Coord { x: 12., y: 15. });
        assert_eq!(Coord::new(2., 3.), Coord { x: 2., y: 3. });
    }

    #[test]
    fn test_add() {
        assert_eq!(Coord::new(1., 2.) + Coord::new(3., 4.), Coord::new(4., 6.));
        assert_eq!(Coord::new(1., -1.) + Coord::new(2., 4.), Coord::new(3., 3.));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Coord::new(1., 2.) - Coord::new(3., 4.), Coord::new(-2., -2.));
        assert_eq!(Coord::new(1., -1.) - Coord::new(2., 4.), Coord::new(-1., -5.));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Coord::new(5., 2.) * 3., Coord::new(15., 6.));
        assert_eq!(Coord::new(3., 4.) * 4., Coord::new(12., 16.));
    }

    #[test]
    fn test_into() {
        assert_eq!(Coord::new(4., 2.), (4., 2.).into());
        assert_eq!(Coord::new(7., -2.), [7., -2.].into());
        assert_eq!(Coord::new(7., -2.) + [1., 5.], Coord::new(8., 3.));
    }

    #[test]
    fn test_default() {
        assert_eq!(Coord::new(0., 0.), Default::default());
    }
}
