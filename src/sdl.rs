use std::{
    f32, thread,
    time::{Duration, Instant},
};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    render::{Canvas, RenderTarget},
};

use crate::{poggle::Poggle, shape::Point};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

const UPDATES_PER_SECOND: u16 = 120;
const FRAMES_PER_SECOND: u16 = 60;

pub trait Render {
    fn render<T>(&self, canvas: &mut Canvas<T>) -> Result<(), String>
    where
        T: RenderTarget;
}

impl From<Point<f32>> for sdl2::rect::Point {
    fn from(value: Point<f32>) -> Self {
        sdl2::rect::Point::new(value.x as i32, value.y as i32)
    }
}

impl From<Point<f32>> for sdl2::rect::FPoint {
    fn from(value: Point<f32>) -> Self {
        sdl2::rect::FPoint::new(value.x, value.y)
    }
}

pub fn run(poggle: &mut Poggle) {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    let window = video
        .window("poggle", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut events = sdl_ctx.event_pump().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RED);
    canvas.clear();
    canvas.present();

    let mut next_update = Instant::now();
    let update_delta = Duration::from_secs(1) / UPDATES_PER_SECOND as u32;

    let mut next_render = Instant::now();
    let render_delta = Duration::from_secs(1) / FRAMES_PER_SECOND as u32;

    let mut is_running = true;
    while is_running {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => is_running = false,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    println!("Clicked at ({}, {})", x, y);
                }
                _ => {}
            }
        }

        if Instant::now() >= next_render {
            next_render += render_delta;
            canvas.set_draw_color(Color::RED);
            canvas.clear();
            poggle.render(&mut canvas).expect("rendering driver failed");
            canvas.present();
        }

        if Instant::now() >= next_update {
            next_update += update_delta;
        }

        thread::sleep(Duration::from_micros(10));
    }
}

pub fn draw_circle<T>(canvas: &mut Canvas<T>, x: f32, y: f32, radius: f32) -> Result<(), String>
where
    T: RenderTarget,
{
    const RESOLUTION: usize = 20;
    let center = Point::new(x, y);
    for i in 0..
    let points: Vec<_> = (0..RESOLUTION)
        .flat_map(|i| {
            let angle = 2.0 * f32::consts::PI * i as f32 / RESOLUTION as f32;
            let (sin, cos) = angle.sin_cos();
            let delta = Point::new(cos, sin) * radius;
            [(center + delta).into()]
        })
        .collect();
    canvas.draw_flines(points.as_slice())?;
    Ok(())
}
