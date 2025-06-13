use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Point<T>
where
    T: Add + Sub + Mul + Div,
{
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

impl<T> Point<T>
where
    T: Add + Sub + Mul + Div,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: Add + Sub<Output = T> + Mul + Div + Into<f64> + Copy,
{
    pub fn length_squared(&self) -> f64 {
        let (x, y) = (self.x.into(), self.y.into());
        x * x + y * y
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl PolarPoint {
    pub const fn new(angle: f32, magnitude: f32) -> Self {
        Self { angle, magnitude }
    }
}

impl<T> Point<T>
where
    T: Add + Sub + Mul + Div + From<u8>,
{
    pub fn zero() -> Self {
        Self {
            x: 0u8.into(),
            y: 0u8.into(),
        }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T> + Sub + Mul + Div,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Point<T>
where
    T: Add<Output = T> + Sub + Mul + Div + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Point<T>
where
    T: Add + Sub<Output = T> + Mul + Div,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Add + Sub + Mul<Output = T> + Div + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> Div<T> for Point<T>
where
    T: Add + Sub + Mul + Div<Output = T> + Copy,
{
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
    Rectangle {
        width: f32,
        height: f32,
        rotation: f32,
    },
    Polygon {
        points: Vec<Point<f32>>,
        rotation: f32,
    },
}

pub struct Body {
    pos: Point<f32>,
    shape: Shape,
}

pub trait Region {
    fn contains(&self, p: Point<f32>) -> bool;
}

impl Region for Body {
    fn contains(&self, p: Point<f32>) -> bool {
        match &self.shape {
            Shape::Circle { radius } => {
                (self.pos - p).length_squared() <= *radius as f64 * *radius as f64
            }
            Shape::Rectangle {
                width,
                height,
                rotation,
            } => todo!(),
            Shape::Polygon { points, rotation } => todo!(),
        }
    }
}
