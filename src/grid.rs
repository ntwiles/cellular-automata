use crate::vectors::vector_2d_int::Vector2DInt;

pub fn grid_index_to_coords(index: usize, grid_width: u32, grid_height: u32) -> Vector2DInt<u32> {
    let x = index % grid_width as usize;
    let y = index / grid_height as usize;

    Vector2DInt {
        x: x as u32,
        y: y as u32,
    }
}

pub fn grid_coords_to_index(position: Vector2DInt<u32>, grid_width: u32) -> usize {
    (position.y * grid_width + position.x) as usize
}
