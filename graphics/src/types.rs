use crate::util::{maxf, minf};

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    #[inline]
    pub fn to_u32(self) -> u32 {
        u32::from_le_bytes([self.r, self.g, self.b, self.a])
    }

    #[inline]
    pub const fn from_floats(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = maxf(0.0, minf(255.0, r * 255.0)) as u8;
        let g = maxf(0.0, minf(255.0, g * 255.0)) as u8;
        let b = maxf(0.0, minf(255.0, b * 255.0)) as u8;
        let a = maxf(0.0, minf(255.0, a * 255.0)) as u8;

        Self { r, g, b, a }
    }
}

pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

pub struct Mesh<const N: usize> {
    positions: [Vec3<f32>; N],
    // this count ultimately will be N / 3, as it's all triangles
    count: usize,
    colour: Colour,
}
