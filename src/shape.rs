use std::ops::{Add, AddAssign, Div, Mul, Sub};

pub trait Number:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
}

impl<T> Number for T where
    T: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
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
}

impl<T: Number> Point<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    pub fn distance_to_squared(self, rhs: Self) -> T {
        (self - rhs).length_squared()
    }
}

impl<T: Number + Into<f64>> Point<T> {
    pub fn length(self) -> f64 {
        self.length_squared().into().sqrt()
    }

    pub fn distance_to(self, rhs: Self) -> f64 {
        self.distance_to_squared(rhs).into().sqrt()
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

impl<T: Number> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Point<u32> {
    pub fn add_signed(self, rhs: Point<i32>) -> Self {
        Self {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
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
