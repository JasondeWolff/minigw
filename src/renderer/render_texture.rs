use super::*;

pub struct RenderTexture {
    texture: GLTexture,
    pbo: GLPBO,
    width: u32,
    height: u32,
    format: u32,
    ty: u32
}

impl RenderTexture {
    pub fn new(format: u32, width: u32, height: u32) -> RenderTexture {
        gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        let texture = GLTexture::new(gl::TEXTURE_2D);
        let mut pbo = GLPBO::new();

        let format = gl::RGBA;
        let size = (width * height * 4 * 4) as usize;
        let ty = gl::FLOAT;

        texture.bind(); {
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            gl_tex_parami(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

            gl_tex_image_2d(gl::RGBA, width as i32, height as i32, format, ty, std::ptr::null());

            pbo.bind();
            pbo.allocate(size);
            pbo.unbind();
        } texture.unbind();

        //gl_pixel_store_i(gl::UNPACK_ALIGNMENT, 4);

        RenderTexture {
            texture,
            pbo,
            width,
            height,
            format,
            ty
        }
    }

    pub fn bind(&self, slot: u32) {
        gl_active_texture(slot);
        self.texture.bind();
    }

    pub fn map<T: Copy>(&self) -> RenderTextureView<T> {
        self.pbo.bind();
        let pixels = self.pbo.map() as *mut T;

        RenderTextureView {
            pixels,
            width: self.width,
            height: self.height,
            format: self.format,
            ty: self.ty,
            elem_count: 4
        }
    }

    pub fn unmap(&self) {
        self.pbo.unmap();
        
        self.texture.bind();
        gl_tex_sub_image_2d(self.width as i32, self.height as i32, self.format, self.ty, std::ptr::null());
        self.texture.unbind();

        self.pbo.unbind();
    }
}