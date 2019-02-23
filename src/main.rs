#[macro_use]
extern crate glium;

mod layer;
mod lib;
mod term;

use crate::lib::Color;

fn main() {
    let mut rt = term::Term::new(80, 25, 12.0, 12.0);

    rt.set(10, 10, Color(255, 0, 0, 255), Color(255, 0, 0, 255));

    rt.set(79, 24, Color(0, 255, 255, 255), Color(0, 255, 255, 255));

    rt.render();

    std::thread::sleep_ms(30000);
}
