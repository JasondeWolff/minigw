pub trait RenderTexturePackedType: Copy + Default {}

#[derive(Default, Clone, Copy, Debug)]
pub struct PackedU32(pub u32);
#[derive(Default, Clone, Copy, Debug)]
pub struct PackedU64(pub u64);
#[derive(Default, Clone, Copy, Debug)]
pub struct PackedU128(pub u128);

impl RenderTexturePackedType for PackedU32 {}
impl RenderTexturePackedType for PackedU64 {}
impl RenderTexturePackedType for PackedU128 {}

pub trait RenderTextureType<T>: Copy + Default {
    fn get_type() -> u32;
}

impl RenderTextureType<PackedU32> for u8 { fn get_type() -> u32 { gl::UNSIGNED_BYTE } }
impl RenderTextureType<PackedU32> for i8 { fn get_type() -> u32 { gl::UNSIGNED_BYTE } }
impl RenderTextureType<PackedU64> for u16 { fn get_type() -> u32 { gl::UNSIGNED_SHORT } }
impl RenderTextureType<PackedU64> for i16 { fn get_type() -> u32 { gl::UNSIGNED_SHORT } }
impl RenderTextureType<PackedU128> for u32 { fn get_type() -> u32 { gl::UNSIGNED_INT } }
impl RenderTextureType<PackedU128> for i32 { fn get_type() -> u32 { gl::UNSIGNED_INT } }
impl RenderTextureType<PackedU128> for f32 { fn get_type() -> u32 { gl::UNSIGNED_INT } }

impl From<&[u8; 4]> for PackedU32 {
    fn from(val: &[u8; 4]) -> Self {
        let (r, g, b, a) = (val[0] as u32, val[1] as u32, val[2] as u32, val[3] as u32);
        PackedU32((a << 24) | (r << 16) | (g << 8) | b)
    }
}

impl From<&[u8; 3]> for PackedU32 {
    fn from(val: &[u8; 3]) -> Self {
        let (r, g, b) = (val[0] as u32, val[1] as u32, val[2] as u32);
        PackedU32((r << 16) | (g << 8) | b)
    }
}

impl From<&[i8; 4]> for PackedU32 {
    fn from(val: &[i8; 4]) -> Self {
        let (r, g, b, a) = (val[0] as u32, val[1] as u32, val[2] as u32, val[3] as u32);
        PackedU32((a << 24) | (r << 16) | (g << 8) | b)
    }
}

impl From<&[u16; 4]> for PackedU64 {
    fn from(val: &[u16; 4]) -> Self {
        let (r, g, b, a) = (val[0] as u64, val[1] as u64, val[2] as u64, val[3] as u64);
        PackedU64((a << 48) | (r << 32) | (g << 16) | b)
    }
}

impl From<&[i16; 4]> for PackedU64 {
    fn from(val: &[i16; 4]) -> Self {
        let (r, g, b, a) = (val[0] as u64, val[1] as u64, val[2] as u64, val[3] as u64);
        PackedU64((a << 48) | (r << 32) | (g << 16) | b)
    }
}

impl From<&[u32; 4]> for PackedU128 {
    fn from(val: &[u32; 4]) -> Self {
        let (r, g, b, a) = (val[0] as u128, val[1] as u128, val[2] as u128, val[3] as u128);
        PackedU128((a << 96) | (r << 64) | (g << 32) | b)
    }
}

impl From<&[i32; 4]> for PackedU128 {
    fn from(val: &[i32; 4]) -> Self {
        let (r, g, b, a) = (val[0] as u128, val[1] as u128, val[2] as u128, val[3] as u128);
        PackedU128((a << 96) | (r << 64) | (g << 32) | b)
    }
}

impl From<&[f32; 4]> for PackedU128 {
    fn from(val: &[f32; 4]) -> Self {
        let (r, g, b, a) = ((val[0] * u32::MAX as f32) as u128, (val[1] * u32::MAX as f32) as u128, (val[2] * u32::MAX as f32) as u128, (val[3] * u32::MAX as f32) as u128);
        PackedU128((a << 96) | (r << 64) | (g << 32) | b)
    }
}