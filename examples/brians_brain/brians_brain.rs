use cellular_automata::{
    automata::Automata,
    grid::{grid_coords_to_index, grid_index_to_coords},
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

use rand::{thread_rng, Rng};

pub struct BriansBrain {
    grid: Vec<u8>,
}

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = 200;
const PIXEL_SCALE: u32 = 4;

impl BriansBrain {
    pub fn new() -> Self {
        let mut grid = vec![0; (GRID_WIDTH * GRID_HEIGHT) as usize];

        let mut i = 0;
        let mut rng = thread_rng();

        while i < grid.len() {
            grid[i] = rng.gen_range(0..3);
            i += 1;
        }

        Self { grid }
    }

    pub fn count_alive_neighbors(&self, grid: &[u8], x: u32, y: u32) -> usize {
        self.check_neighbor(grid, x, y, 0, -1)
            + self.check_neighbor(grid, x, y, 0, 1)
            + self.check_neighbor(grid, x, y, -1, 0)
            + self.check_neighbor(grid, x, y, 1, 0)
            + self.check_neighbor(grid, x, y, -1, -1)
            + self.check_neighbor(grid, x, y, 1, -1)
            + self.check_neighbor(grid, x, y, -1, 1)
            + self.check_neighbor(grid, x, y, 1, 1)
    }

    fn check_neighbor(&self, grid: &[u8], x: u32, y: u32, ox: i32, oy: i32) -> usize {
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

        (grid[grid_coords_to_index(nx, ny, GRID_WIDTH)] == 2) as usize
    }
}

impl Automata for BriansBrain {
    fn update(&mut self) {
        let mut grid_next = self.grid.clone();

        for i in 0..self.grid.len() {
            let (x, y) = grid_index_to_coords(i, GRID_WIDTH, GRID_HEIGHT);

            let neighbors = self.count_alive_neighbors(&self.grid, x, y);

            grid_next[i] = match (self.grid[i], neighbors) {
                (2, _) => 1,
                (1, _) => 0,
                (0, 2) => 2,
                (l, _) => l,
            };
        }

        self.grid = grid_next;
    }

    fn before_render(&self) {}

    fn render(&self, pixels: &mut [u8]) {
        let viewport_width = GRID_WIDTH * PIXEL_SCALE;
        let viewport_height = GRID_HEIGHT * PIXEL_SCALE;

        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            let (vx, vy) = viewport_index_to_coords(i, viewport_width, viewport_height);
            let (x, y) = viewport_to_grid(vx, vy, PIXEL_SCALE);

            let index = grid_coords_to_index(x, y, GRID_WIDTH);

            let color = match self.grid[index] {
                2 => [0x0, 0x99, 0x77, 0xff],
                1 => [0x0, 0xdd, 0xdd, 0xff],
                0 => [0x0, 0x22, 0x44, 0xff],
                _ => unreachable!(),
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
