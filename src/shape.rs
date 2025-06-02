use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Point<T>
where
    T: Add + Sub + Mul + Div,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Add + Sub + Mul + Div,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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

impl Add<Point<i32>> for Point<u32> {
    type Output = Self;

    fn add(self, rhs: Point<i32>) -> Self::Output {
        Self::new(
            self.x.wrapping_add_signed(rhs.x),
            self.y.wrapping_add_signed(rhs.y),
        )
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
