use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderTextureResizing {
    Resizable,
    ResizableScaled(f32),
    NonResizable
}

pub struct RenderTexture<T: RenderTextureType> {
    texture: GLTexture,
    pbo: Vec<GLPBO>,
    pbo_idx: usize,
    src_width: u32,
    src_height: u32,
    width: u32,
    height: u32,
    ty: u32,

    pixels: Vec<T>,
    use_pbo: bool,
    resizing: RenderTextureResizing
}

impl<T: RenderTextureType> RenderTexture<T> {
    pub(crate) fn new(width: u32, height: u32, use_pbo: bool, resizing: RenderTextureResizing) -> RenderTexture<T> {
        let src_width = width;
        let src_height = height;
        let (width, height) = Self::get_sized_dims(width, height, resizing);

        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        let size = (width * height * std::mem::size_of::<T>() as u32 * 3) as usize;

        let texture = GLTexture::new(gl::TEXTURE_2D);
        let mut pbo = vec![GLPBO::new(), GLPBO::new()];
        let pixels = vec![T::default(); size / std::mem::size_of::<T>()];
        let ty = T::get_type();

        texture.bind(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

            gl_tex_image_2d(gl::RGB, width as i32, height as i32, gl::RGB, ty, std::ptr::null());

            for pbo in &mut pbo {
                pbo.bind();
                pbo.allocate(size);
                pbo.unbind();
            }
        } texture.unbind();

        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);

        RenderTexture {
            texture,
            pbo,
            pbo_idx: 0,
            src_width,
            src_height,
            width,
            height,
            ty,
            pixels,
            use_pbo,
            resizing
        }
    }

    pub(crate) fn bind(&self, slot: u32) {
        gl_active_texture(slot);
        self.texture.bind();
    }

    pub(crate) fn write(&mut self) {
        if self.use_pbo {
            self.pbo[self.pbo_idx].bind();
            let pixels = self.pbo[self.pbo_idx].map() as *mut T;
            unsafe {
                pixels.copy_from_nonoverlapping(self.pixels.as_ptr(), self.pixels.len());
            }
            self.pbo[self.pbo_idx].unmap();

            gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

            self.texture.bind();
            gl_tex_sub_image_2d(self.width as i32, self.height as i32, gl::RGB, self.ty, std::ptr::null());
            self.texture.unbind();

            gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);

            self.pbo[self.pbo_idx].unbind();

            self.pbo_idx = (self.pbo_idx + 1) % self.pbo.len();
        } else {
            self.texture.bind();
            gl_tex_sub_image_2d(self.width as i32, self.height as i32, gl::RGB, self.ty, self.pixels.as_ptr() as *const std::ffi::c_void);
            self.texture.unbind();
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.src_width = width;
        self.src_height = height;

        if let RenderTextureResizing::NonResizable = self.resizing {
            return;
        }
        
        self.internal_resize(width, height);
    }

    fn internal_resize(&mut self, width: u32, height: u32) {
        self.src_width = width;
        self.src_height = height;
        let (width, height) = Self::get_sized_dims(width, height, self.resizing);
        self.width = width;
        self.height = height;

        gl_finish();
        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        let size = (width * height * std::mem::size_of::<T>() as u32 * 3) as usize;

        self.texture = GLTexture::new(gl::TEXTURE_2D);
        self.pbo = vec![GLPBO::new(), GLPBO::new()];
        self.pbo_idx = 0;
        self.pixels = vec![T::default(); size / std::mem::size_of::<T>()];

        self.texture.bind(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

            gl_tex_image_2d(gl::RGB, width as i32, height as i32, gl::RGB, self.ty, std::ptr::null());

            for pbo in &mut self.pbo {
                pbo.bind();
                pbo.allocate(size);
                pbo.unbind();
            }
        } self.texture.unbind();

        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);
    }

    fn get_sized_dims(width: u32, height: u32, resizing: RenderTextureResizing) -> (u32, u32) {
        match resizing {
            RenderTextureResizing::ResizableScaled(scale) => {
                ((width as f32 * scale) as u32, (height as f32 * scale) as u32)
            },
            _ => (width, height)
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &[T; 3] {
        let start = ((y * self.width + x) * 3) as usize;
        let end = start + 3;
        self.pixels[start..end].try_into().unwrap()
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: &[T; 3]) {
        let start = ((y * self.width + x) * 3) as usize;
        let end = start + 3;

        let mut j = 0;
        for i in start..end {
            self.pixels[i] = value[j];
            j += 1;
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_resizing_mode(&self) -> RenderTextureResizing {
        self.resizing
    }

    pub fn set_resizing_mode(&mut self, resizing: RenderTextureResizing) {
        self.resizing = resizing;
        self.internal_resize(self.src_width, self.src_height);
    }
}