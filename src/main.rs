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
    let mut rt = term::Term::new("tileset.png", 80, 40, 12.0, 12.0);
    let mut rng = rand::thread_rng();

    rt.set_back_color(Color(40, 80, 190, 255));

    for x in 0..80 {
        for y in 0..40 {
            let r = rng.gen_range(0, 255);
            let g = rng.gen_range(0, 255);
            let b = rng.gen_range(0, 255);
            let color = Color(r, g, b, 255);

            rt.set(((y * 80 + x) % 255) as u8, x, y, 0, color);
        }
    }

    rt.set(176, 0, 0, 0, Color(255, 0, 0, 255));
    rt.set('g' as u8, 0, 0, 1, Color(255, 0, 0, 255));
    rt.set('^' as u8, 0, 0, 2, Color(255, 0, 0, 255));

    let mut closed = false;
    let mut clear = false;
    let mut t = 0.0;
    while !closed {
        rt.render();

        if clear {
            rt.clear();
            clear = false;
        }

        t += 0.1;
        rt.set_ext('X' as u8, 1, t, 1, t, 3, Color(130, 88, 250, 255));

        rt.events_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input {
                    glutin::KeyboardInput {
                        virtual_keycode, ..
                    } => match virtual_keycode {
                        Some(glutin::VirtualKeyCode::C) => clear = true,
                        _ => println!("pressed {:?}", virtual_keycode),
                    },
                },
                _ => (),
            },
            _ => (),
        });

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
