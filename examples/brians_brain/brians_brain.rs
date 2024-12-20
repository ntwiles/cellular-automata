use cellular_automata::{
    automata::Automata,
    grid::{grid_coords_to_index, grid_index_to_coords},
    vectors::vector_2d_int::Vector2DInt,
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

    pub fn count_alive_neighbors(&self, grid: &[u8], pos: Vector2DInt<u32>) -> usize {
        self.check_neighbor(grid, pos, Vector2DInt { x: 0, y: -1 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: 0, y: 1 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: -1, y: 0 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: 1, y: 0 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: -1, y: -1 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: 1, y: -1 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: -1, y: 1 })
            + self.check_neighbor(grid, pos, Vector2DInt { x: 1, y: 1 })
    }

    fn check_neighbor(
        &self,
        grid: &[u8],
        pos: Vector2DInt<u32>,
        offset: Vector2DInt<i32>,
    ) -> usize {
        let neighbor: Vector2DInt<i32> = pos.to_i32() + offset;

        if neighbor.x < 0 || neighbor.y < 0 {
            return 0;
        }

        let neighbor = Vector2DInt {
            x: neighbor.x as u32,
            y: neighbor.y as u32,
        };

        if neighbor.x >= GRID_WIDTH || neighbor.y >= GRID_HEIGHT {
            return 0;
        }

        (grid[grid_coords_to_index(neighbor, GRID_WIDTH)] == 2) as usize
    }
}

impl Automata<(u32, u32)> for BriansBrain {
    fn update(&mut self) {
        let mut grid_next = self.grid.clone();

        for i in 0..self.grid.len() {
            let pos = grid_index_to_coords(i, GRID_WIDTH, GRID_HEIGHT);
            let neighbors = self.count_alive_neighbors(&self.grid, pos);

            grid_next[i] = match (self.grid[i], neighbors) {
                (2, _) => 1,
                (1, _) => 0,
                (0, 2) => 2,
                (l, _) => l,
            };
        }

        self.grid = grid_next;
    }

    fn before_render(&self) -> (u32, u32) {
        (GRID_WIDTH * PIXEL_SCALE, GRID_HEIGHT * PIXEL_SCALE)
    }

    fn render(&self, context: &(u32, u32), i: usize, pixel: &mut [u8]) {
        let (viewport_width, viewport_height) = context;

        let viewport_position = viewport_index_to_coords(i, *viewport_width, *viewport_height);
        let position = viewport_to_grid(viewport_position, PIXEL_SCALE);
        let index = grid_coords_to_index(position, GRID_WIDTH);

        let color = match self.grid[index] {
            2 => [0x0, 0x99, 0x77, 0xff],
            1 => [0x0, 0xdd, 0xdd, 0xff],
            0 => [0x0, 0x22, 0x44, 0xff],
            _ => unreachable!(),
        };

        pixel.copy_from_slice(&color);
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
