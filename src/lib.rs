#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

pub struct Size(i32, i32);

impl Color {
    pub(crate) fn to_floats(&self) -> [f32; 4] {
        [
            f32::from(self.0) / 255.0,
            f32::from(self.1) / 255.0,
            f32::from(self.2) / 255.0,
            f32::from(self.3) / 255.0,
        ]
    }
}
