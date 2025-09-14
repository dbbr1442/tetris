pub struct Rect {
    position: (u32, u32),
    size: (u32, u32),
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            position: (x, y),
            size: (w, h),
        }
    }

    pub fn get_data(&self) -> (f32, f32, f32, f32) {
        (self.position.0 as f32,
         self.position.1 as f32,
         self.size.0 as f32,
         self.size.1 as f32,
        )
    }
}
