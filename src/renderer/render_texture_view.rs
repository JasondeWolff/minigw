pub struct RenderTextureView<T: Copy> {
    pub(super) pixels: Vec<T>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) elem_count: isize
}

impl<T: Copy> RenderTextureView<T> {
    pub fn get_pixel(&self, x: u32, y: u32) -> &[T] {
        let start = ((y * self.width + x) * self.elem_count as u32) as usize;
        let end = start + self.elem_count as usize;
        &self.pixels[start..end]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: &[T]) {
        let start = ((y * self.width + x) * self.elem_count as u32) as usize;
        let end = start + self.elem_count as usize;

        let mut j = 0;
        for i in start..end {
            self.pixels[i] = value[j];
            j += 1;
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}