#[macro_use]
extern crate glium;
extern crate rand;

mod layer;
mod lib;
mod term;

use crate::lib::Color;
use glium::glutin;
use rand::Rng;

fn main() {
    let mut rt = term::Term::new(80, 40, 12.0, 12.0);
    let mut rng = rand::thread_rng();

    for x in 0..80 {
        for y in 0..40 {
            let r = rng.gen_range(0, 255);
            let g = rng.gen_range(0, 255);
            let b = rng.gen_range(0, 255);
            let color = Color(r, g, b, 255);

            rt.set(((y * 80 + x) % 255) as u8, x, y, color, color);
        }
    }

    rt.set(
        'b' as u8,
        0,
        0,
        Color(255, 0, 0, 255),
        Color(255, 0, 0, 255),
    );

    let mut closed = false;
    while !closed {
        rt.render();

        rt.events_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
