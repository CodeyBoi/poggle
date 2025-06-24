use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub},
};

pub trait Number:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
{
}

impl<T> Number for T where
    T: Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + PartialOrd
{
}

#[derive(Clone, Copy, Debug)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy)]
pub struct PolarPoint {
    pub angle: f32,
    pub magnitude: f32,
}

impl From<PolarPoint> for Point<f32> {
    fn from(value: PolarPoint) -> Self {
        let (sin, cos) = value.angle.sin_cos();
        Self::new(cos, sin) * value.magnitude
    }
}

impl From<Point<f32>> for PolarPoint {
    fn from(value: Point<f32>) -> Self {
        Self::new(value.y.atan2(value.x), value.x.hypot(value.y))
    }
}

impl<T: Number> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn to(self, rhs: Self) -> Self {
        rhs - self
    }

    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    pub fn distance_to_squared(self, rhs: Self) -> T {
        self.to(rhs).length_squared()
    }

    pub fn is_longer_than(self, rhs: T) -> bool {
        self.length_squared() > rhs * rhs
    }
}

impl<T: Number + Into<f32>> Point<T> {
    pub fn length(self) -> f32 {
        self.length_squared().into().sqrt()
    }

    pub fn distance_to(self, rhs: Self) -> f32 {
        self.distance_to_squared(rhs).into().sqrt()
    }

    pub fn normalized(self) -> Point<f32> {
        Point::new(self.x.into(), self.y.into()) / self.length()
    }
}

impl PolarPoint {
    pub const fn new(angle: f32, magnitude: f32) -> Self {
        Self { angle, magnitude }
    }
}

impl<T: Number + From<u8>> Point<T> {
    pub fn zero() -> Self {
        Self {
            x: 0u8.into(),
            y: 0u8.into(),
        }
    }
}

impl<T: Number> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Number + AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Number> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Number> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Number + MulAssign> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Number> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Number + Neg<Output = T>> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl<T: Number + Eq> Eq for Point<T> {}

impl<T: Number + PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Number + Display> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        self.x.fmt(f)?;
        write!(f, ", ")?;
        self.y.fmt(f)?;
        write!(f, ")")
    }
}

impl Point<u32> {
    pub fn add_signed(self, rhs: Point<i32>) -> Self {
        Self {
            x: self.x.wrapping_add_signed(rhs.x),
            y: self.y.wrapping_add_signed(rhs.y),
        }
    }
}

pub enum Shape {
    Circle {
        radius: f32,
    },
    Polygon {
        points: Vec<Point<f32>>,
        rotation: f32,
    },
}

pub struct Body {
    pub pos: Point<f32>,
    pub shape: Shape,
}

pub trait Region {
    fn contains(&self, p: Point<f32>) -> bool;
}

impl Region for Body {
    fn contains(&self, p: Point<f32>) -> bool {
        match &self.shape {
            Shape::Circle { radius } => (self.pos - p).length_squared() <= *radius * *radius,
            Shape::Polygon { points, rotation } => todo!(),
        }
    }
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let midpoint = -b / (2.0 * a);

    // If B^2 - 4AC < 0 then no real solution exists
    let v = b.powi(2) - 4.0 * a * c;
    if v < 0.0 {
        return None;
    }

    let delta = v.sqrt() / (2.0 * a);

    Some((midpoint + delta, midpoint - delta))
}

#[cfg(test)]
mod tests {
    use crate::shape::Point;

    #[test]
    fn test_add() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
        let c = Point::new(5, 6);

        assert_eq!(a + b, Point::new(4, 6));
        assert_eq!(b + a, Point::new(4, 6));
        assert_eq!(a + c, Point::new(6, 8));
        assert_eq!(b + c, Point::new(8, 10));
    }

    #[test]
    fn test_dot() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
        let c = Point::new(5, 6);

        assert_eq!(a.dot(b), 11);
        assert_eq!(b.dot(a), 11);
        assert_eq!(a.dot(c), 17);
        assert_eq!(b.dot(c), 39);
    }

    #[test]
    fn test_length() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(3.0, 4.0);
        let c = Point::new(5.0, 6.0);

        assert!((a.length_squared() - 5.0f32).abs() < f32::EPSILON);
        assert!((b.length_squared() - 25.0f32).abs() < f32::EPSILON);
        assert!((c.length_squared() - 61.0f32).abs() < f32::EPSILON);

        assert!((a.length() - 5.0f32.sqrt()).abs() < f32::EPSILON);
        assert!((b.length() - 5.0f32).abs() < f32::EPSILON);
        assert!((c.length() - 61.0f32.sqrt()).abs() < f32::EPSILON);
    }
}
