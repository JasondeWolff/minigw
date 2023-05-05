pub struct RenderTextureView<T: Copy> {
    pub(super) pixels: *mut T,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) format: u32,
    pub(super) ty: u32,
    pub(super) elem_count: isize
}

impl<T: Copy> RenderTextureView<T> {
    pub fn get_pixel(&self, x: u32, y: u32) -> &[T] {
        unsafe {
            let offset = (y * self.width + x) as isize;
            let data_ptr = self.pixels.offset(offset * self.elem_count);
            std::slice::from_raw_parts(data_ptr, self.elem_count as usize)
        }
    }

    pub fn set_pixel(&self, x: u32, y: u32, value: &[T]) {
        unsafe {
            let offset = (y * self.width + x) as isize;
            let data_ptr = self.pixels.offset(offset * self.elem_count);
            let elems = std::slice::from_raw_parts_mut(data_ptr, self.elem_count as usize);
            
            for i in 0..self.elem_count {
                elems[i as usize] = value[i as usize];
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}