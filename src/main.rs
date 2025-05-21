use sdl2::pixels::Color;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

struct Poggle {
    pegs: Vec<Peg>,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

struct Peg {
    pos: Point,
    shape: Shape,
    is_hit: bool,
}

enum Shape {
    Circle {
        radius: f32,
    },
    Rectangle {
        width: f32,
        height: f32,
        rotation: f32,
    },
    Polygon {
        points: Vec<Point>,
        rotation: f32,
    },
}

struct Ball {
    pos: Point,
    velocity: Point,
}

impl Poggle {
    pub fn run(&mut self) {
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

        while self.is_running {
            self.handle_events(&mut events);

            if Instant::now() >= next_timer_decrement {
                next_timer_decrement += duration_per_timer_decrement;
                if self.dt > 0 {
                    self.dt -= 1;
                }
                if self.st > 0 {
                    self.st -= 1;
                }
            }

            if Instant::now() >= self.next_display_interrupt {
                self.next_display_interrupt += duration_per_frame;
                self.render(&mut canvas).expect("rendering driver failed");
            }

            if Instant::now() >= next_update {
                next_update += duration_per_update;

                if !self.is_suspended {
                    self.cycle();
                }
            }

            thread::sleep(Duration::from_micros(10));
        }
    }
}

fn main() {}
