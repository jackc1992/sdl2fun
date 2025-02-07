#[inline]
pub const fn float4_to_u32(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let r = maxf(0.0, minf(255.0, r * 255.0)) as u8;
    let g = maxf(0.0, minf(255.0, g * 255.0)) as u8;
    let b = maxf(0.0, minf(255.0, b * 255.0)) as u8;
    let a = maxf(0.0, minf(255.0, a * 255.0)) as u8;

    u32::from_le_bytes([r, g, b, a])
}

#[inline]
pub const fn minf(f1: f32, f2: f32) -> f32 {
    if f1 < f2 { f1 } else { f2 }
}

#[inline]
pub const fn maxf(f1: f32, f2: f32) -> f32 {
    if f1 > f2 { f1 } else { f2 }
}
