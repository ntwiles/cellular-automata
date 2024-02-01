pub fn viewport_index_to_coords(
    index: usize,
    viewport_width: u32,
    viewport_height: u32,
) -> (u32, u32) {
    let x = index % viewport_width as usize;
    let y = index / viewport_height as usize;

    (x as u32, y as u32)
}

pub fn viewport_to_grid(x: u32, y: u32, render_pixel_scale: u32) -> (u32, u32) {
    (x / render_pixel_scale, y / render_pixel_scale)
}
