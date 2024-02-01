pub fn grid_index_to_coords(index: usize, grid_width: u32, grid_height: u32) -> (u32, u32) {
    let x = index % grid_width as usize;
    let y = index / grid_height as usize;

    (x as u32, y as u32)
}

pub fn grid_coords_to_index(x: u32, y: u32, grid_width: u32) -> usize {
    (y * grid_width + x) as usize
}
