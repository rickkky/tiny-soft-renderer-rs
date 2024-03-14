pub struct Viewport {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: u32,
    pub height: u32,
}
impl Viewport {
    pub fn new(offset_x: i32, offset_y: i32, width: u32, height: u32) -> Self {
        Viewport {
            offset_x,
            offset_y,
            width,
            height,
        }
    }
}
