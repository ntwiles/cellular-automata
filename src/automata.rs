pub trait Automata {
    fn update(&mut self);
    fn render(&self, pixels: &mut [u8]);
    fn grid_width(&self) -> u32;
    fn grid_height(&self) -> u32;
    fn render_pixel_scale(&self) -> u32;
}
