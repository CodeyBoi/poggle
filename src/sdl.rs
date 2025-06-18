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

pub const UPDATES_PER_SECOND: u16 = 60;
const FRAMES_PER_SECOND: u16 = 60;

pub trait Render {
    fn render<T>(&self, canvas: &mut Canvas<T>) -> Result<(), String>
    where
        T: RenderTarget;
}

impl From<Point<u32>> for sdl2::rect::Point {
    fn from(value: Point<u32>) -> Self {
        sdl2::rect::Point::new(value.x as i32, value.y as i32)
    }
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
    let mut last_update = Instant::now() - update_delta;

    let mut next_render = Instant::now();
    let render_delta = Duration::from_secs(1) / FRAMES_PER_SECOND as u32;
    let mut target_start = None;
    let mut target_end = None;

    let mut is_running = true;
    let mut mouse_down = false;
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
                    mouse_down = true;
                    target_start = Some(Point::new(x as f32, y as f32));
                }
                Event::MouseMotion { x, y, .. } => {
                    let p = Point::new(x as f32, y as f32);
                    if mouse_down {
                        poggle.shoot(p, Point::zero());
                    }
                    target_end = Some(p);
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    mouse_down = false;
                    if let Some(origin) = target_start {
                        let end = Point::new(x as f32, y as f32);
                        let velocity = origin - end;
                        poggle.shoot(origin, velocity);
                        (target_start, target_end) = (None, None);
                    }
                }
                _ => {}
            }
        }

        let now = Instant::now();

        if now >= next_render {
            next_render += render_delta;
            canvas.set_draw_color(Color::GRAY);
            canvas.clear();
            poggle.render(&mut canvas).expect("rendering driver failed");
            if let (Some(start), Some(end)) = (target_start, target_end) {
                canvas.set_draw_color(Color::RED);
                canvas
                    .draw_line(start, end)
                    .expect("rendering driver failed");
            }
            canvas.present();
        }

        if now >= next_update {
            poggle.update(update_delta);
            next_update += update_delta;
            last_update += update_delta;
        }

        thread::sleep(Duration::from_micros(10));
    }
}

fn get_octant_offsets(radius: u32) -> Vec<Point<i32>> {
    let mut offsets = Vec::with_capacity((radius as usize + 1) * 2);
    let (mut dx, mut dy) = (0, radius as i32);

    offsets.push(Point::new(dx, dy));
    let mut d = 3 - (2 * radius as i32);

    while dx < dy {
        dx += 1;
        d += if d < 0 {
            4 * dx + 2
        } else {
            dy -= 1;
            4 * (dx - dy) + 6
        };
        offsets.push(Point::new(dx, dy));
    }
    offsets
}

pub fn draw_circle_filled<T>(
    canvas: &mut Canvas<T>,
    x: u32,
    y: u32,
    radius: u32,
) -> Result<(), String>
where
    T: RenderTarget,
{
    let center = Point::new(x, y);
    for offset in get_octant_offsets(radius) {
        let (dx, dy) = (offset.x, offset.y);
        for d in [
            Point::new(dx, dy),
            Point::new(dy, dx),
            Point::new(dy, -dx),
            Point::new(dx, -dy),
        ] {
            let other = Point::new(-d.x, d.y);
            canvas.draw_line(center.add_signed(other), center.add_signed(d))?;
        }
    }
    Ok(())
}

pub fn draw_circle<T>(canvas: &mut Canvas<T>, x: u32, y: u32, radius: u32) -> Result<(), String>
where
    T: RenderTarget,
{
    let center = Point::new(x, y);
    for offset in get_octant_offsets(radius) {
        let (dx, dy) = (offset.x, offset.y);
        for d in [
            Point::new(dx, dy),
            Point::new(dx, -dy),
            Point::new(-dx, dy),
            Point::new(-dx, -dy),
            Point::new(dy, dx),
            Point::new(dy, -dx),
            Point::new(-dy, dx),
            Point::new(-dy, -dx),
        ] {
            canvas.draw_point(center.add_signed(d))?;
        }
    }
    Ok(())
}
