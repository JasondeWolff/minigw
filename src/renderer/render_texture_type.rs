pub trait RenderTextureType: Copy + Default {
    fn get_type() -> u32;
}

impl RenderTextureType for u8 { fn get_type() -> u32 { gl::UNSIGNED_BYTE } }
impl RenderTextureType for i8 { fn get_type() -> u32 { gl::BYTE } }
impl RenderTextureType for u16 { fn get_type() -> u32 { gl::UNSIGNED_SHORT } }
impl RenderTextureType for i16 { fn get_type() -> u32 { gl::SHORT } }
impl RenderTextureType for u32 { fn get_type() -> u32 { gl::UNSIGNED_INT } }
impl RenderTextureType for i32 { fn get_type() -> u32 { gl::INT } }
impl RenderTextureType for f32 { fn get_type() -> u32 { gl::FLOAT } }