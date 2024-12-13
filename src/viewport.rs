use crate::vector_2d::Vector2D;

pub fn viewport_index_to_coords(
    index: usize,
    viewport_width: u32,
    viewport_height: u32,
) -> Vector2D<u32> {
    let x = index % viewport_width as usize;
    let y = index / viewport_height as usize;

    Vector2D {
        x: x as u32,
        y: y as u32,
    }
}

pub fn viewport_to_grid(position: Vector2D<u32>, render_pixel_scale: u32) -> Vector2D<u32> {
    Vector2D {
        x: position.x / render_pixel_scale,
        y: position.y / render_pixel_scale,
    }
}
