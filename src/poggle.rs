use std::{f32::consts, time::Duration};

use sdl2::pixels::Color;

use crate::{
    sdl::{self, Render, draw_circle, draw_circle_filled},
    shape::{Body, Point, PolarPoint, Region, Shape, solve_quadratic},
};

const GRAVITY: Point<f32> = Point::new(0.0, 550.0);

pub struct Poggle {
    balls: Vec<Ball>,
    pegs: Vec<Peg>,
    tick: u64,
}

pub struct Target {
    pos: Point<f32>,
    dir: Point<f32>,
}

pub struct Ball {
    pos: Point<f32>,
    velocity: Point<f32>,
    start: Point<f32>,
}

impl Ball {
    const RADIUS: f32 = 6.0;
    const ELASTICITY: f32 = 0.9;
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
    pub fn new(pos: Point<f32>, velocity: Point<f32>) -> Self {
        Self {
            pos,
            velocity,
            start: pos,
        }
    }

    fn will_collide(&self, other: &Body, time: Duration) -> Option<Point<f32>> {
        match &other.shape {
            Shape::Circle { radius } => {
                let movement = self.velocity * time.as_secs_f32();

                // Check if collision is even possible during this timestep
                if self.pos.distance_to_squared(other.pos)
                    > (radius + Ball::RADIUS + movement.length()).powi(2)
                {
                    return None;
                }

                // With line -> y = mx + k and circle -> (x - p)^2 + (y - q)^2 = r^2 we get
                // Ax^2 + Bx + C = 0 where A = m^2 + 1, B = 2(mk - mq - p), and
                // C = q^2 - r^2 + p^2 - 2kq + k^2. Solutions are then given by
                // x' = (-B ± sqrt(B^2 - 4AC)) / 2A.
                let m = movement.y / movement.x;
                let k = self.pos.y - self.pos.x * m;

                let p = other.pos.x;
                let q = other.pos.y;
                let r = radius + Ball::RADIUS;

                if movement.x.abs() < 1.0 {
                    // In this case we have x = t which gives us
                    // y^2 - 2qy + (p^2 + q^2 - r^2 - 2dp + d^2)
                    let d = self.pos.x;
                    let a = 1.0;
                    let b = -2.0 * q;
                    let c = p.powi(2) - r.powi(2) + q.powi(2) - 2.0 * d * p + d.powi(2);

                    let (y1, y2) = solve_quadratic(a, b, c)?;
                    let y_new = if (y1 - self.pos.y).abs() < (y2 - self.pos.y).abs() {
                        y1
                    } else {
                        y2
                    };

                    if movement.is_longer_than(165.0 * time.as_secs_f32())
                        && self.velocity.y.signum() != (y_new - self.pos.y).signum()
                    {
                        return None;
                    }

                    return Some(Point::new(self.pos.x, y_new));
                }

                let x_new = {
                    let a = m.powi(2) + 1.0;
                    let b = 2.0 * (m * k - m * q - p);
                    let c = q.powi(2) - r.powi(2) + p.powi(2) - 2.0 * k * q + k.powi(2);

                    // Find the closest of the two points
                    let (x1, x2) = solve_quadratic(a, b, c)?;
                    if (x1 - self.pos.x).abs() < (x2 - self.pos.x).abs() {
                        x1
                    } else {
                        x2
                    }
                };

                // Check the direction is correct
                if movement.is_longer_than(165.0 * time.as_secs_f32())
                    && movement.x.signum() != (x_new - self.pos.x).signum()
                {
                    return None;
                }

                // As y = mx + k
                let collision = Point::new(x_new, m * x_new + k);

                // Check if collision will happen during the allotted time
                if self.pos.distance_to_squared(collision) * 0.99 > movement.length_squared() {
                    return None;
                }

                Some(collision)
            }
            Shape::Polygon { points, rotation } => todo!(),
        }
    }

    fn potential_energy(&self) -> f32 {
        (sdl::WINDOW_HEIGHT as f32 - self.pos.y) * GRAVITY.y
    }

    fn total_energy(&self) -> f32 {
        self.velocity.kinetic_energy() + self.potential_energy()
    }
}

impl Poggle {
    pub fn new() -> Self {
        let spacing = 75.0;
        let pegs = Self::generate_grid(
            Point::new(100.0, 400.0),
            Point::new(sdl::WINDOW_WIDTH as f32 - 100.0, 700.0),
            spacing,
        )
        .into_iter()
        .chain(Self::generate_grid(
            Point::new(100.0, 400.0) + Point::new(spacing / 2.0, spacing / 2.0),
            Point::new(sdl::WINDOW_WIDTH as f32 - 100.0, 700.0)
                - Point::new(spacing / 2.0, spacing / 2.0),
            spacing,
        ))
        .collect();

        let amount = 200;
        let space = 11.0;
        let center = sdl::WINDOW_WIDTH as f32 / 2.0;
        let positions = (-amount..amount + 1)
            .map(|i| Point::new(center + i as f32 / amount as f32 * space - 15.0, 100.0));
        let balls = positions.map(|pos| Ball::new(pos, Point::zero())).collect();

        // let pegs = vec![Peg {
        //     body: Body {
        //         pos: Point::new(
        //             sdl::WINDOW_WIDTH as f32 / 2.0,
        //             sdl::WINDOW_HEIGHT as f32 / 2.0,
        //         ),
        //         shape: Shape::Circle { radius: 50.0 },
        //     },
        //     is_hit: false,
        //     peg_type: PegType::Standard,
        // }];

        Self {
            balls,
            pegs,
            tick: 0,
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
        self.balls.push(Ball::new(origin, velocity));
    }

    pub fn update(&mut self, delta: Duration) {
        self.balls.retain_mut(|ball| {
            if ball.pos.y > sdl::WINDOW_HEIGHT as f32 + Ball::RADIUS {
                return false;
            }

            let d = delta.as_secs_f32();
            ball.velocity += GRAVITY * d;
            ball.pos += ball.velocity * d;

            for peg in &mut self.pegs {
                if peg.body.extend(Ball::RADIUS).contains(ball.pos) {
                    ball.pos = match &peg.body.shape {
                        Shape::Circle { radius } => {
                            peg.body.pos
                                + peg
                                    .body
                                    .pos
                                    .to(ball.pos)
                                    .with_length(*radius + Ball::RADIUS)
                        }
                        Shape::Polygon { points, rotation } => todo!(),
                    };
                }
                if let Some(collision) = ball.will_collide(&peg.body, delta) {
                    let start_velocity = ball.velocity;

                    let distance_to_travel = ball.velocity.length() * delta.as_secs_f32();
                    let reflect = peg.body.pos.to(collision).normalized();

                    // this is not entirely correct
                    ball.velocity += reflect * reflect.dot(ball.velocity).abs() * 2.0;
                    ball.velocity = ball
                        .velocity
                        .with_length(start_velocity.length() * Ball::ELASTICITY);

                    ball.pos = collision
                        + ball.velocity.normalized()
                            * (distance_to_travel - ball.pos.distance_to(collision));
                    peg.is_hit = true;

                    // if ball.velocity.length_squared() > start_velocity.length_squared() {
                    //     println!("Tick {}: ball got {:.0}% speed when bouncing off peg at {} (angle {:.2}, {:.2} -> {:.2}, EK {:.0} -> {:.0})",self.tick, ball.velocity.length() / start_velocity.length() * 100.0, peg.body.pos, std::convert::Into::<PolarPoint>::into(peg.body.pos.to(ball.pos)).angle * 180.0 / consts::PI, start_velocity, ball.velocity, start_velocity.kinetic_energy(), ball.velocity.kinetic_energy());
                    // }

                    break;
                }
            }

            if ball.pos.x < Ball::RADIUS / 2.0
                || ball.pos.x > sdl::WINDOW_WIDTH as f32 - Ball::RADIUS / 2.0
            {
                ball.velocity.x *= -1.0;
            }

            // for peg in &self.pegs {
            //     if peg.body.extend(Ball::RADIUS).contains(ball.pos) {
            //         println!("Ball is inside peg at {}", peg.body.pos);
            //     }
            // }

            true
        });

        if self.balls.is_empty() {
            for peg in &mut self.pegs {
                peg.is_hit = false;
            }
        }

        self.tick += 1;
    }
}

impl Render for Poggle {
    fn render<T>(&self, canvas: &mut sdl2::render::Canvas<T>) -> Result<(), String>
    where
        T: sdl2::render::RenderTarget,
    {
        for ball in &self.balls {
            ball.render(canvas)?;
        }

        for peg in &self.pegs {
            peg.render(canvas)?;
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
        let start = Ball::new(self.start, Point::zero());
        if self.total_energy() > start.total_energy() {
            // println!(
            //     "Ball starting at {} has an energy of {:.2} (started at {:.2})",
            //     self.start,
            //     self.total_energy(),
            //     start.total_energy()
            // );
            canvas.set_draw_color(Color::GREEN);
        } else {
            canvas.set_draw_color(Color::RED);
        }

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
        canvas.draw_line(self.pos, self.pos + self.velocity * 0.10)?;
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
            PegType::Standard => {
                if self.is_hit {
                    Color::YELLOW
                } else {
                    Color::RGB(0, 0, 255)
                }
            }
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
                // canvas.set_draw_color(Color::GREEN);
                // draw_circle(
                //     canvas,
                //     self.body.pos.x as u32,
                //     self.body.pos.y as u32,
                //     *radius as u32 + Ball::RADIUS as u32,
                // )?;
            }
            Shape::Polygon { points, rotation } => todo!(),
        }
        Ok(())
    }
}
