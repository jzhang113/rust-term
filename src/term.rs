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
    cell_width: f32,
    cell_height: f32,
    vertices: Vec<Vertex>,
    events_loop: glutin::EventsLoop,
    display: glium::Display,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    shade_color: [f32; 4],
}

implement_vertex!(Vertex, position, tex_coords, shade_color);

impl Term {
    pub fn new(width: u32, height: u32, cell_width: f32, cell_height: f32) -> Term {
        let tot_width = (width as f32) * cell_width;
        let tot_height = (height as f32) * cell_height;
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

        Term {
            front: Layer::new(size),
            back: Layer::new(size),
            width: width,
            height: height,
            cell_width: cell_width,
            cell_height: cell_height,
            vertices: vec![],
            events_loop: events_loop,
            display: display,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, fcol: Color, bcol: Color) {
        let index = (y * self.width + x) as usize;
        // TODO: how to double buffer?
        self.front.cells[index].color = fcol;
    }

    pub fn render(&mut self) {
        // TODO: double buffer
        let buf = &self.front;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                self::Term::draw_cell(
                    &mut self.vertices,
                    &buf.cells[index],
                    x,
                    y,
                    self.cell_width,
                    self.cell_height,
                )
            }
        }

        // build vertex buffers
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &self.vertices).unwrap();

        // TODO: compute index buffer
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

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
                // color = texture(tex, old_tex_coords);
                color = old_color;
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
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    fn draw_cell(
        vertices: &mut Vec<Vertex>,
        cell: &Cell,
        x: u32,
        y: u32,
        cell_width: f32,
        cell_height: f32,
    ) {
        let left = (x as f32) * cell_width + cell.dx;
        let top = (y as f32) * cell_height + cell.dy;
        let right = left + cell_width + cell.dx;
        let bottom = top + cell_height + cell.dy;

        let Color(r, g, b, a) = cell.color;
        let shade_color = [
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
            f32::from(a) / 255.0,
        ];

        // set up vertices + texcoords
        vertices.push(Vertex {
            position: [left, top],
            tex_coords: [0.0, 1.0],
            shade_color: shade_color,
        });
        vertices.push(Vertex {
            position: [left, bottom],
            tex_coords: [0.0, 1.0],
            shade_color: shade_color,
        });
        vertices.push(Vertex {
            position: [right, bottom],
            tex_coords: [0.0, 1.0],
            shade_color: shade_color,
        });
        vertices.push(Vertex {
            position: [right, top],
            tex_coords: [0.0, 1.0],
            shade_color: shade_color,
        });
    }
}
