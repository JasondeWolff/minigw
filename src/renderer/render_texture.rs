use super::*;

pub struct RenderTexture<T: Copy + Default> {
    texture: GLTexture,
    pbo: Vec<GLPBO>,
    pbo_idx: usize,
    width: u32,
    height: u32,
    format: u32,
    ty: u32,
    size: usize,

    view: Option<RcCell<RenderTextureView<T>>>
}

impl<T: Copy + Default> RenderTexture<T> {
    pub fn new(format: u32, width: u32, height: u32) -> RenderTexture<T> {
        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        let texture = GLTexture::new(gl::TEXTURE_2D);
        let mut pbo = vec![GLPBO::new(), GLPBO::new()];

        let format = gl::RGBA;
        let size = (width * height * 4 * 4) as usize;
        let ty = gl::FLOAT;

        texture.bind(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

            gl_tex_image_2d(gl::RGBA, width as i32, height as i32, format, ty, std::ptr::null());

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
            format,
            ty,
            size,
            view: None
        }
    }

    pub fn bind(&self, slot: u32) {
        gl_active_texture(slot);
        self.texture.bind();
    }

    pub fn map(&mut self) -> RcCell<RenderTextureView<T>> {
        self.pbo[self.pbo_idx].bind();

        match &self.view {
            Some(view) => view.clone(),
            None => {
                let view = RcCell::new(RenderTextureView {
                    pixels: vec![T::default(); self.size / 4],
                    width: self.width,
                    height: self.height,
                    elem_count: 4
                });
        
                self.view = Some(view.clone());
                view
            }
        }
    }

    pub fn unmap(&mut self) {
        let pixels = self.pbo[self.pbo_idx].map() as *mut T;
        unsafe {
            let view = self.view.as_ref().unwrap().as_ref();
            pixels.copy_from_nonoverlapping(view.pixels.as_ptr(), view.pixels.len());
        }
        self.pbo[self.pbo_idx].unmap();
        
        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        self.texture.bind();
        gl_tex_sub_image_2d(self.width as i32, self.height as i32, self.format, self.ty, std::ptr::null());
        self.texture.unbind();

        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);

        self.pbo[self.pbo_idx].unbind();

        self.pbo_idx = (self.pbo_idx + 1) % self.pbo.len();
    }
}