pub trait Automata<T> {
    fn update(&mut self);
    fn before_render(&self) -> T;
    fn render(&self, context: &T, i: usize, pixel: &mut [u8]);
    fn grid_width(&self) -> u32;
    fn grid_height(&self) -> u32;
    fn render_pixel_scale(&self) -> u32;
}
