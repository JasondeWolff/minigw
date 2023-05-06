use super::*;

pub struct RenderTexture<T: RenderTextureType> {
    texture: GLTexture,
    pbo: Vec<GLPBO>,
    pbo_idx: usize,
    width: u32,
    height: u32,
    ty: u32,
    size: usize,

    view: Option<RcCell<RenderTextureView<T>>>,
    use_pbo: bool
}

impl<T: RenderTextureType> RenderTexture<T> {
    pub fn new(width: u32, height: u32, use_pbo: bool) -> RenderTexture<T> {
        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        let texture = GLTexture::new(gl::TEXTURE_2D);
        let mut pbo = vec![GLPBO::new(), GLPBO::new()];

        let size = (width * height * std::mem::size_of::<T>() as u32 * 3) as usize;
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
            width,
            height,
            ty,
            size,
            view: None,
            use_pbo
        }
    }

    pub fn bind(&self, slot: u32) {
        gl_active_texture(slot);
        self.texture.bind();
    }

    pub fn map(&mut self) -> RcCell<RenderTextureView<T>> {
        match &self.view {
            Some(view) => view.clone(),
            None => {
                let view = RcCell::new(RenderTextureView {
                    pixels: vec![T::default(); self.size / std::mem::size_of::<T>()],
                    width: self.width,
                    height: self.height
                });
        
                self.view = Some(view.clone());
                view
            }
        }
    }

    pub fn unmap(&mut self) {
        if self.use_pbo {
            self.pbo[self.pbo_idx].bind();
            let pixels = self.pbo[self.pbo_idx].map() as *mut T;
            unsafe {
                let view = self.view.as_ref().unwrap().as_ref();
                pixels.copy_from_nonoverlapping(view.pixels.as_ptr(), view.pixels.len());
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
            let view = self.view.as_ref().unwrap().as_ref();

            self.texture.bind();
            gl_tex_sub_image_2d(self.width as i32, self.height as i32, gl::RGB, self.ty, view.pixels.as_ptr() as *const std::ffi::c_void);
            self.texture.unbind();
        }
    }
}