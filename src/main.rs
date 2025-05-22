use poggle::Poggle;

mod poggle;
mod sdl;
mod shape;

fn main() {
    let mut poggle = Poggle::new();

    sdl::run(&mut poggle);
}
