use core::f32;
use std::{f32::consts::PI, time::Duration};

use sdl2::pixels::Color;

use crate::{
    sdl::{Render, draw_circle, draw_circle_filled},
    shape::{Point, PolarPoint, Shape},
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
        draw_circle_filled(canvas, self.pos.x as u32, self.pos.y as u32, 7)?;
        canvas.set_draw_color(Color::BLACK);
        draw_circle(canvas, self.pos.x as u32, self.pos.y as u32, 7)?;
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
            Shape::Rectangle {
                width,
                height,
                rotation,
            } => todo!(),
            Shape::Polygon { points, rotation } => todo!(),
        }
        Ok(())
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
    }
}
