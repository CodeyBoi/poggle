use std::time::Duration;

use sdl2::pixels::Color;

use crate::{
    sdl::{self, Render, draw_circle, draw_circle_filled},
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
    const RADIUS: f32 = 6.0;
}

pub struct Peg {
    body: Body,
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
    fn will_collide(&self, other: &Body, time: Duration) -> Option<Point<f32>> {
        match &other.shape {
            Shape::Circle { radius } => {
                // With line -> y = mx + k and circle -> (x - p)^2 + (y - q)^2 = r^2 we get
                // Ax^2 + Bx + C = 0 where A = m^2 + 1, B = 2(mk - mq - p), and
                // C = q^2 - r^2 + p^2 - 2kq + k^2. Solutions are then given by
                // x' = (-B ± sqrt(B^2 - 4AC)) / 2A.
                let m = self.velocity.y / self.velocity.x; // Will be INF if velocity.x is zero
                let k = self.pos.y - self.pos.x * m;

                let x_new = {
                    let p = other.pos.x;
                    let q = other.pos.y;
                    let r = radius + Ball::RADIUS;

                    let a = m.powi(2) + 1.0;
                    let b = 2.0 * (m * k - m * q - p);
                    let c = q.powi(2) - r.powi(2) + p.powi(2) - 2.0 * k * q + k.powi(2);

                    let midpoint = -b / (2.0 * a);

                    // If B^2 - 4AC < 0 then no real solution exists
                    let v = b.powi(2) - 4.0 * a * c;
                    if v < 0.0 {
                        return None;
                    }

                    let delta = v.sqrt() / (2.0 * a);

                    // Find the closest of the two points
                    let (x1, x2) = (midpoint + delta, midpoint - delta);
                    if (x1 - self.pos.x).abs() < (x2 - self.pos.x).abs() {
                        x1
                    } else {
                        x2
                    }
                };

                // Check the direction is correct
                if self.velocity.x.signum() != (x_new - self.pos.x).signum() {
                    return None;
                }

                // As y = mx + k
                let collision = Point::new(x_new, m * x_new + k);

                // Check if collision will happen during the allotted time
                if self.pos.distance_to_squared(collision)
                    > (self.velocity * time.as_secs_f32() * 1.5).length_squared()
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
        let pegs = Self::generate_grid(
            Point::new(100.0, 400.0),
            Point::new(sdl::WINDOW_WIDTH as f32 - 100.0, 700.0),
            50.0,
        );

        Self {
            ball: None,
            pegs,
            target: None,
        }
    }

    fn generate_grid(origin: Point<f32>, end: Point<f32>, spacing: f32) -> Vec<Peg> {
        let mut out = Vec::new();
        let mut point = origin;
        while point.y <= end.y {
            out.push(Peg {
                body: Body {
                    pos: point,
                    shape: Shape::Circle { radius: 6.0 },
                },
                is_hit: false,
                peg_type: PegType::Standard,
            });

            point.x += spacing;
            if point.x > end.x {
                point.x = origin.x;
                point.y += spacing;
            }
        }
        out
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

            for peg in &self.pegs {
                if let Some(collision) = ball.will_collide(&peg.body, delta) {
                    let distance_to_travel = ball.velocity.length() * delta.as_secs_f32();
                    let reflect = peg.body.pos.to(collision).normalized();
                    ball.velocity += -reflect * reflect.dot(ball.velocity) * 2.0;
                    ball.pos = collision
                        + ball.velocity.normalized()
                            * (distance_to_travel - ball.pos.distance_to(collision));
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
        for peg in &self.pegs {
            peg.render(canvas)?;
        }

        if let Some(ball) = &self.ball {
            ball.render(canvas)?;
        }

        // canvas.set_draw_color(Color::GREEN);
        // if let Some(ball) = &self.ball {
        //     for peg in &self.pegs {
        //         if let Some(collision) = ball.will_collide(
        //             &peg.body,
        //             Duration::from_micros(1_000_000 / sdl::UPDATES_PER_SECOND as u64),
        //         ) {
        //             canvas.draw_line(
        //                 Point::new(0.0f32, collision.y),
        //                 Point::new(10000.0f32, collision.y),
        //             )?;
        //             canvas.draw_line(
        //                 Point::new(collision.x, 0.0f32),
        //                 Point::new(collision.x, 10000.0f32),
        //             )?;
        //         }
        //     }
        // }

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
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_line(
            self.pos,
            self.pos
                + self.velocity
                    * Duration::from_micros(1_000_000 / sdl::UPDATES_PER_SECOND as u64)
                        .as_secs_f32(),
        )?;
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
        match &self.body.shape {
            Shape::Circle { radius } => {
                draw_circle_filled(
                    canvas,
                    self.body.pos.x as u32,
                    self.body.pos.y as u32,
                    *radius as u32,
                )?;
                canvas.set_draw_color(Color::BLACK);
                draw_circle(
                    canvas,
                    self.body.pos.x as u32,
                    self.body.pos.y as u32,
                    *radius as u32,
                )?;
            }
            Shape::Polygon { points, rotation } => todo!(),
        }
        Ok(())
    }
}
