use crate::vectors::vector_2d_int::Vector2DInt;

pub fn viewport_index_to_coords(
    index: usize,
    viewport_width: u32,
    viewport_height: u32,
) -> Vector2DInt<u32> {
    let x = index % viewport_width as usize;
    let y = index / viewport_height as usize;

    Vector2DInt {
        x: x as u32,
        y: y as u32,
    }
}

pub fn viewport_to_grid(position: Vector2DInt<u32>, render_pixel_scale: u32) -> Vector2DInt<u32> {
    Vector2DInt {
        x: position.x / render_pixel_scale,
        y: position.y / render_pixel_scale,
    }
}
