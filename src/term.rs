extern crate image;

use glium::glutin;
use glium::Surface;

use crate::layer::Cell;
use crate::layer::Layer;
use crate::lib::Color;

pub struct Term {
    front: Layer,
    back: Layer,
    width: u32,
    height: u32,
    cell_width: f64,
    cell_height: f64,
    count: u32,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    texture: glium::Texture2d,
    display: glium::Display,
    pub events_loop: glutin::EventsLoop,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 2],
    tex_coords: [f64; 2],
    shade_color: [f32; 4],
}

implement_vertex!(Vertex, position, tex_coords, shade_color);

impl Term {
    pub fn new(width: u32, height: u32, cell_width: f64, cell_height: f64) -> Term {
        let tot_width = f64::from(width) * cell_width;
        let tot_height = f64::from(height) * cell_height;
        let size = (width * height) as usize;

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize::new(
                tot_width.into(),
                tot_height.into(),
            ))
            .with_title("Hello world");

        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // get texture
        // TODO: load image from a given path
        use std::io::Cursor;
        let image = image::load(Cursor::new(&include_bytes!("tileset.png")[..]), image::PNG)
            .unwrap()
            .to_rgba();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(&display, image).unwrap();

        Term {
            front: Layer::new(size),
            back: Layer::new(size),
            width: width,
            height: height,
            cell_width: cell_width,
            cell_height: cell_height,
            count: 0,
            vertices: vec![],
            indices: vec![],
            texture: texture,
            events_loop: events_loop,
            display: display,
        }
    }

    // TODO: handle codes over 255
    pub fn set(&mut self, code: u8, x: u32, y: u32, fcol: Color, bcol: Color) {
        let index = (y * self.width + x) as usize;
        // TODO: how to double buffer?
        let tile = &mut self.front.cells[index];
        tile.color = fcol;
        tile.code = code;
    }

    pub fn render(&mut self) {
        // TODO: double buffer
        let buf = self.front.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                self.draw_cell(&buf.cells[index], x, y)
            }
        }

        // build vertex and index buffers
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &self.vertices).unwrap();
        let indices = glium::index::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &self.indices,
        )
        .unwrap();

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            in vec2 tex_coords;
            in vec4 shade_color;
            out vec2 old_pos;
            out vec2 old_tex_coords;
            out vec4 old_color;

            void main() {
                old_pos = position;
                old_tex_coords = tex_coords;
                old_color = shade_color;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 old_pos;
            in vec2 old_tex_coords;
            in vec4 old_color;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = old_color * texture(tex, old_tex_coords);
            }
        "#;

        let program = glium::Program::from_source(
            &self.display,
            vertex_shader_src,
            fragment_shader_src,
            None,
        )
        .unwrap();

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! {
                    tex: &self.texture
                },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    fn draw_cell(&mut self, cell: &Cell, x: u32, y: u32) {
        let tot_width = f64::from(self.width) * self.cell_width;
        let tot_height = f64::from(self.height) * self.cell_height;

        let left = (f64::from(x) * self.cell_width + cell.dx) * 2.0 / tot_width - 1.0;
        let bottom = (f64::from(y) * self.cell_height + cell.dy) * 2.0 / tot_height - 1.0;
        let right = left + (cell.dx + self.cell_width) * 2.0 / tot_width;
        let top = bottom + (cell.dy + self.cell_height) * 2.0 / tot_height;

        let Color(r, g, b, a) = cell.color;
        let shade_color = [
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
            f32::from(a) / 255.0,
        ];

        let sxt = 1.0 / 16.0;
        let row = 16 - (cell.code / 16);
        let col = cell.code % 16;

        let tex_left = sxt * f64::from(col);
        let tex_right = sxt * f64::from(col + 1);
        let tex_top = sxt * f64::from(row);
        let tex_bottom = sxt * f64::from(row - 1);

        // set up vertices + texcoords
        &self.vertices.push(Vertex {
            position: [left, top],
            tex_coords: [tex_left, tex_top],
            shade_color: shade_color,
        });
        &self.vertices.push(Vertex {
            position: [left, bottom],
            tex_coords: [tex_left, tex_bottom],
            shade_color: shade_color,
        });
        &self.vertices.push(Vertex {
            position: [right, bottom],
            tex_coords: [tex_right, tex_bottom],
            shade_color: shade_color,
        });
        &self.vertices.push(Vertex {
            position: [right, top],
            tex_coords: [tex_right, tex_top],
            shade_color: shade_color,
        });

        &self.indices.push(self.count + 0);
        &self.indices.push(self.count + 1);
        &self.indices.push(self.count + 2);
        &self.indices.push(self.count + 0);
        &self.indices.push(self.count + 2);
        &self.indices.push(self.count + 3);

        self.count += 4;
    }
}