use crate::vector_2d::Vector2D;

pub fn grid_index_to_coords(index: usize, grid_width: u32, grid_height: u32) -> Vector2D<u32> {
    let x = index % grid_width as usize;
    let y = index / grid_height as usize;

    Vector2D {
        x: x as u32,
        y: y as u32,
    }
}

pub fn grid_coords_to_index(position: Vector2D<u32>, grid_width: u32) -> usize {
    (position.y * grid_width + position.x) as usize
}
