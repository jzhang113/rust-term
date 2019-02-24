extern crate rand;
extern crate rust_term;

use glium::glutin;
use rand::Rng;
use rust_term::color::Color;

fn main() {
    let width = 40;
    let height = 40;
    let mut map = [['#' as u8; 40]; 40];

    let mut rt = rust_term::Term::new("tileset.png", width, height, 12.0, 12.0);
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let mut x = rng.gen_range(0, width);
        let mut y = rng.gen_range(0, height);

        for _j in 0..100 {
            map[y as usize][x as usize] = '.' as u8;

            let dx = rng.gen_range(0, 2);
            let dy = rng.gen_range(0, 2);

            if x == 0 && dx == 0 {
                x = 0;
            } else {
                x = x + dx - 1;
            }

            if y == 0 && dy == 0 {
                y = 0;
            } else {
                y = y + dy - 1;
            }
        }
    }

    let mut player_x = 0;
    let mut player_y = 0;

    let mut closed = false;
    while !closed {
        rt.clear();
        draw_map(&map, &mut rt);
        rt.set('@' as u8, player_x, player_y, 1, Color(255, 255, 255, 255));
        rt.render();

        rt.events_loop.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input {
                    glutin::KeyboardInput {
                        state,
                        virtual_keycode,
                        ..
                    } => match state {
                        glutin::ElementState::Pressed => match virtual_keycode {
                            Some(glutin::VirtualKeyCode::Up) => {
                                if player_y < 39 {
                                    player_y += 1;
                                }
                            }
                            Some(glutin::VirtualKeyCode::Down) => {
                                if player_y > 0 {
                                    player_y -= 1;
                                }
                            }
                            Some(glutin::VirtualKeyCode::Left) => {
                                if player_x > 0 {
                                    player_x -= 1;
                                }
                            }
                            Some(glutin::VirtualKeyCode::Right) => {
                                if player_x < 39 {
                                    player_x += 1;
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                },
                _ => (),
            },
            _ => (),
        });

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn draw_map(map: &[[u8; 40]; 40], rt: &mut rust_term::Term) {
    for y in 0..40 {
        for x in 0..40 {
            let symb = map[y][x];
            let col;

            let wall_col = Color(191, 82, 9, 255);
            let floor_col = Color(214, 194, 143, 255);
            let other_col = Color(255, 255, 255, 255);

            if symb == ('#' as u8) {
                col = wall_col;
            } else if symb == ('.' as u8) {
                col = floor_col;
            } else {
                col = other_col;
            }

            rt.set(map[y][x], x as u32, y as u32, 0, col);
        }
    }
}
