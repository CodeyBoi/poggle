use core::f32;
use std::{f32::consts::PI, time::Duration};

use sdl2::pixels::Color;

use crate::{
    sdl::{Render, draw_circle, draw_circle_filled},
    shape::{Body, Point, PolarPoint, Region, Shape},
};

const GRAVITY: Point<f32> = Point::new(0.0, 400.0);

pub struct Poggle {
    ball: Option<Ball>,
    pegs: Vec<Peg>,
    target: Option<Target>,
}

pub struct Target {
    pos: Point<f32>,
    dir: Point<f32>,
}

pub struct Ball {
    pos: Point<f32>,
    velocity: Point<f32>,
}

impl Ball {
    const RADIUS: f32 = 7.0;
}

pub struct Peg {
    pos: Point<f32>,
    shape: Shape,
    is_hit: bool,
    peg_type: PegType,
}

pub enum PegType {
    Standard,
    Target,
    PointBoost,
    PowerUp(PowerUp),
}

pub enum PowerUp {
    SuperGuide,
    MultiBall,
    Pyramid,
    Explosion,
    SpookyBall,
    MagicWheel,
    Flippers,
    Fireball,
    FlowerPower,
    Zen,
}

impl Ball {
    fn will_collide(&self, other: Body, time: Duration) -> Option<Point<f32>> {
        match other.shape {
            Shape::Circle { radius } => {
                // With line -> y = mx + k and circle -> (x - p)^2 + (y - q)^2 = r^2 we get
                // Ax^2 + Bx + C = 0 where A = m^2 + 1, B = 2(mk - mq - p), and
                // C = q^2 - r^2 + p^2 - 2kq + k^2. Solutions are then given by
                // x' = (-B ± sqrt(B^2 - 4AC)) / 2A.
                let m = self.velocity.y / self.velocity.x;
                let k = self.pos.y - self.pos.x * m;

                let p = other.pos.x;
                let q = other.pos.y;
                let r = radius + Ball::RADIUS;

                let a = m.powi(2) + 1.0;
                let b = 2.0 * (m * k - m * q - p);
                let c = q.powi(2) - r.powi(2) + p.powi(2) - 2.0 * k * q + k.powi(2);

                let midpoint = b / (2.0 * a);
                let delta = (b.powi(2) - 4.0 * a * c) / 2.0 * a;
                let (x1, x2) = (midpoint + delta, midpoint - delta);

                let x_new = if (x1 - self.pos.x).abs() < (x2 - self.pos.x).abs() {
                    x1
                } else {
                    x2
                };

                if self.velocity.x.signum() != (x_new - self.pos.x).signum() {
                    return None;
                }

                let collision = Point::new(x_new, m * x_new + c);

                if self.pos.distance_to_squared(collision)
                    > (self.velocity * time.as_secs_f32()).length_squared()
                {
                    return None;
                }

                Some(collision)
            }
            Shape::Polygon { points, rotation } => todo!(),
        }
    }
}

impl Poggle {
    pub fn new() -> Self {
        let pegs = vec![
            Peg {
                pos: Point::new(100.0, 150.0),
                shape: Shape::Circle { radius: 5.0 },
                is_hit: false,
                peg_type: PegType::Standard,
            },
            Peg {
                pos: Point::new(150.0, 150.0),
                shape: Shape::Circle { radius: 5.0 },
                is_hit: false,
                peg_type: PegType::Target,
            },
            Peg {
                pos: Point::new(200.0, 150.0),
                shape: Shape::Circle { radius: 5.0 },
                is_hit: false,
                peg_type: PegType::PowerUp(PowerUp::SuperGuide),
            },
            Peg {
                pos: Point::new(250.0, 150.0),
                shape: Shape::Circle { radius: 5.0 },
                is_hit: false,
                peg_type: PegType::PointBoost,
            },
        ];
        Self {
            ball: None,
            pegs,
            target: None,
        }
    }

    pub fn shoot(&mut self, origin: Point<f32>, velocity: Point<f32>) {
        self.ball = Some(Ball {
            pos: origin,
            velocity,
        });
    }

    pub fn update(&mut self, delta: Duration) {
        if let Some(ball) = &mut self.ball {
            let d = delta.as_secs_f32();
            ball.velocity += GRAVITY * d;
            ball.pos += ball.velocity * d;
        }

        if let Some(ball) = &self.ball {
            for peg in &self.pegs {
                match &peg.shape {
                    Shape::Circle { radius } => {
                        let body = Body {
                            pos: peg.pos,
                            shape: Shape::Circle {
                                radius: radius + Ball::RADIUS,
                            },
                        };
                        if body.contains(ball.pos) {
                            println!("COLLISION AT {:?}", ball.pos);
                        }
                    }
                    Shape::Polygon { points, rotation } => todo!(),
                }
            }
        }
    }
}

impl Render for Poggle {
    fn render<T>(&self, canvas: &mut sdl2::render::Canvas<T>) -> Result<(), String>
    where
        T: sdl2::render::RenderTarget,
    {
        if let Some(ball) = &self.ball {
            ball.render(canvas)?;
        }

        for peg in &self.pegs {
            peg.render(canvas)?;
        }

        Ok(())
    }
}

impl Render for Ball {
    fn render<T>(&self, canvas: &mut sdl2::render::Canvas<T>) -> Result<(), String>
    where
        T: sdl2::render::RenderTarget,
    {
        canvas.set_draw_color(Color::RED);
        draw_circle_filled(
            canvas,
            self.pos.x as u32,
            self.pos.y as u32,
            Ball::RADIUS as u32,
        )?;
        canvas.set_draw_color(Color::BLACK);
        draw_circle(
            canvas,
            self.pos.x as u32,
            self.pos.y as u32,
            Ball::RADIUS as u32,
        )?;
        canvas.set_draw_color(Color::MAGENTA);
        canvas.draw_line(self.pos, self.pos + self.velocity)?;
        Ok(())
    }
}

impl Render for Peg {
    fn render<T>(&self, canvas: &mut sdl2::render::Canvas<T>) -> Result<(), String>
    where
        T: sdl2::render::RenderTarget,
    {
        let color = match self.peg_type {
            PegType::Standard => Color::BLUE,
            PegType::Target => Color::RED,
            PegType::PointBoost => Color::MAGENTA,
            PegType::PowerUp(_) => Color::GREEN,
        };
        canvas.set_draw_color(color);
        match &self.shape {
            Shape::Circle { radius } => {
                draw_circle_filled(canvas, self.pos.x as u32, self.pos.y as u32, *radius as u32)?;
                canvas.set_draw_color(Color::BLACK);
                draw_circle(canvas, self.pos.x as u32, self.pos.y as u32, *radius as u32)?;
            }
            Shape::Polygon { points, rotation } => todo!(),
        }
        Ok(())
    }
}
