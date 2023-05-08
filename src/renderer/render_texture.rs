use super::*;

/// The resizing behaviour of a `RenderTexture`.
/// - `Resizable` the render texture will be resized to the window size.
/// - `ResizableScaled(f32)` the render texture will be resized to the window size scaled by a `f32` factor.
/// - `NonResizable` the render texture will not be resized when the window resizes **or** when `RenderTexture<T>::resize(u32, u32)` is called.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderTextureResizing {
    Resizable,
    ResizableScaled(f32),
    NonResizable
}

/// RenderTexture containing RGB pixel data with every element in the form of `T`.
pub struct RenderTexture<T: RenderTextureType> {
    texture: GLTexture,
    pbo: GLPBO,
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
        let mut pbo = GLPBO::new();
        let pixels = vec![T::default(); size / std::mem::size_of::<T>()];
        let ty = T::get_type();

        texture.bind(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

            gl_tex_image_2d(gl::RGB, width as i32, height as i32, gl::RGB, ty, std::ptr::null());

            pbo.bind();
            pbo.allocate(size);
            pbo.unbind();
        } texture.unbind();

        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);

        RenderTexture {
            texture,
            pbo,
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

    pub(crate) fn async_write(&mut self) {
        if self.use_pbo {
            self.pbo.bind();
            let pixels = self.pbo.map() as *mut T;
            unsafe {
                pixels.copy_from_nonoverlapping(self.pixels.as_ptr(), self.pixels.len());
            }
            self.pbo.unmap();
            self.pbo.unbind();
        }
    }

    pub(crate) fn flush_write(&mut self) {
        if self.use_pbo {
            self.pbo.bind();
            gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

            self.texture.bind();
            gl_tex_sub_image_2d(self.width as i32, self.height as i32, gl::RGB, self.ty, std::ptr::null());
            self.texture.unbind();

            gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);
            self.pbo.unbind();
        } else {
            self.texture.bind();
            gl_tex_sub_image_2d(self.width as i32, self.height as i32, gl::RGB, self.ty, self.pixels.as_ptr() as *const std::ffi::c_void);
            self.texture.unbind();
        }
    }

    /// Resize the render texture, will clear the pixel buffer to 0.
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
        self.pbo = GLPBO::new();
        self.pixels = vec![T::default(); size / std::mem::size_of::<T>()];

        self.texture.bind(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

            gl_tex_image_2d(gl::RGB, width as i32, height as i32, gl::RGB, self.ty, std::ptr::null());

            self.pbo.bind();
            self.pbo.allocate(size);
            self.pbo.unbind();
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

    /// Get pixel at coordinates `[x, y]`.
    /// Always make sure `x >= 0 && x < width` AND `y >= 0 && y < height`.
    #[inline(always)]
    pub fn get_pixel(&self, x: u32, y: u32) -> (T, T, T) {
        let i = ((y * self.width + x) * 3) as usize;
        (self.pixels[i], self.pixels[i + 1], self.pixels[i + 2])
    }

    /// Set pixel at coordinates `[x, y]`.
    /// Always make sure `x >= 0 && x < width` AND `y >= 0 && y < height`.
    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, r: T, g: T, b: T) {
        let i = ((y * self.width + x) * 3) as usize;
        self.pixels[i] = r;
        self.pixels[i + 1] = g;
        self.pixels[i + 2] = b;
    }

    /// Get width.
    #[inline(always)]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get height.
    #[inline(always)]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get current resizing mode.
    #[inline(always)]
    pub fn get_resizing_mode(&self) -> RenderTextureResizing {
        self.resizing
    }

    /// Set current resizing mode.
    pub fn set_resizing_mode(&mut self, resizing: RenderTextureResizing) {
        self.resizing = resizing;
        self.internal_resize(self.src_width, self.src_height);
    }
}