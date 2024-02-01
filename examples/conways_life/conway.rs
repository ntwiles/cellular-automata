use rand::{thread_rng, Rng};

use cellular_automata::{
    automata::Automata,
    grid::{grid_coords_to_index, grid_index_to_coords},
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = 200;
const PIXEL_SCALE: u32 = 4;

const VIEWPORT_WIDTH: u32 = GRID_WIDTH * PIXEL_SCALE;
const VIEWPORT_HEIGHT: u32 = GRID_HEIGHT * PIXEL_SCALE;

pub struct Conway {
    grid: Vec<bool>,
}

impl Conway {
    pub fn new() -> Self {
        let mut grid = vec![false; (GRID_WIDTH * GRID_HEIGHT) as usize];

        let mut i = 0;
        let mut rng = thread_rng();

        while i < grid.len() {
            grid[i] = rng.gen_bool(0.5);
            i += 1;
        }

        Self { grid }
    }

    fn check_neighbor(&self, grid: &[bool], x: u32, y: u32, ox: i32, oy: i32) -> usize {
        let nx = x as i32 + ox;
        let ny = y as i32 + oy;

        if nx < 0 || ny < 0 {
            return 0;
        }

        let nx = nx as u32;
        let ny = ny as u32;

        if nx >= GRID_WIDTH || ny >= GRID_HEIGHT {
            return 0;
        }

        grid[grid_coords_to_index(nx, ny, GRID_WIDTH)] as usize
    }

    pub fn count_alive_neighbors(&self, grid: &[bool], x: u32, y: u32) -> usize {
        self.check_neighbor(grid, x, y, 0, -1)
            + self.check_neighbor(grid, x, y, 0, 1)
            + self.check_neighbor(grid, x, y, -1, 0)
            + self.check_neighbor(grid, x, y, 1, 0)
            + self.check_neighbor(grid, x, y, -1, -1)
            + self.check_neighbor(grid, x, y, 1, -1)
            + self.check_neighbor(grid, x, y, -1, 1)
            + self.check_neighbor(grid, x, y, 1, 1)
    }
}

impl Automata for Conway {
    fn update(&mut self) {
        let mut grid_next = self.grid.clone();

        for i in 0..self.grid.len() {
            let (x, y) = grid_index_to_coords(i, GRID_WIDTH, GRID_HEIGHT);

            let neighbors = self.count_alive_neighbors(&self.grid, x, y);

            grid_next[i] = match (self.grid[i], neighbors) {
                (true, 0 | 1) => false,
                (true, 2 | 3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false,
            };
        }

        self.grid = grid_next;
    }

    fn render(&self, pixels: &mut [u8]) {
        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            let (vx, vy) = viewport_index_to_coords(i, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
            let (x, y) = viewport_to_grid(vx, vy, PIXEL_SCALE);

            let index = grid_coords_to_index(x, y, GRID_WIDTH);

            let color = if self.grid[index] {
                [0x0, 0x99, 0x11, 0xff]
            } else {
                [0x0, 0x22, 0x11, 0xff]
            };

            pixel.copy_from_slice(&color);
        }
    }

    fn grid_width(&self) -> u32 {
        GRID_WIDTH
    }

    fn grid_height(&self) -> u32 {
        GRID_HEIGHT
    }

    fn render_pixel_scale(&self) -> u32 {
        PIXEL_SCALE
    }
}
