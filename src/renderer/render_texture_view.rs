pub struct RenderTextureView<T: Copy> {
    pub(super) pixels: Vec<T>,
    pub(super) width: u32,
    pub(super) height: u32
}

impl<T: Copy> RenderTextureView<T> {
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}