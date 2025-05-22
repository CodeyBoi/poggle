use core::f32;
use std::f32::consts::PI;

use sdl2::pixels::Color;

use crate::{
    sdl::{Render, draw_circle},
    shape::{Point, Shape},
};

pub struct Poggle {
    ball: Option<Ball>,
    pegs: Vec<Peg>,
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
        Ok(())
    }
}

impl Render for Peg {
    fn render<T>(&self, canvas: &mut sdl2::render::Canvas<T>) -> Result<(), String>
    where
        T: sdl2::render::RenderTarget,
    {
        canvas.set_draw_color(Color::BLUE);
        match &self.shape {
            Shape::Circle { radius } => draw_circle(canvas, self.pos.x, self.pos.y, *radius)?,
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
        let pegs = vec![Peg {
            pos: Point::new(100.0, 150.0),
            shape: Shape::Circle { radius: 4.0 },
            is_hit: false,
            peg_type: PegType::Standard,
        }];
        Self { ball: None, pegs }
    }

    pub fn shoot(&mut self, angle: f32) {
        let angle = angle.clamp(-PI, PI);
    }
}
