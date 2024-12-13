use rand::{thread_rng, Rng};

use cellular_automata::{
    automata::Automata,
    grid::{grid_coords_to_index, grid_index_to_coords},
    vectors::vector_2d_int::Vector2DInt,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};
pub struct RockPaperScissors {
    grid: Vec<usize>,
}

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = 200;
const PIXEL_SCALE: u32 = 4;

const VIEWPORT_WIDTH: u32 = GRID_WIDTH * PIXEL_SCALE;
const VIEWPORT_HEIGHT: u32 = GRID_HEIGHT * PIXEL_SCALE;

impl RockPaperScissors {
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

    fn is_beaten_by_neighbor(
        &self,
        grid: &[usize],
        pos: Vector2DInt<u32>,
        offset: Vector2DInt<i32>,
    ) -> usize {
        let neighbor = pos.to_i32() + offset;

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

        let cell = grid_coords_to_index(pos, GRID_WIDTH);
        let neighbor = grid_coords_to_index(neighbor, GRID_WIDTH);

        // 0 = rock, 1 = paper, 2 = scissors
        match (grid[cell], grid[neighbor]) {
            (0, 1) => 1,
            (1, 2) => 1,
            (2, 0) => 1,
            _ => 0,
        }
    }

    // TODO: This is very similar to the analagous function in Conway. Refactor.
    // We can create functions get_moore_neighbors() and get_von_neumann_neighbors()
    // instead of this.
    pub fn is_beaten(&self, grid: &[usize], pos: Vector2DInt<u32>) -> bool {
        let defeats = self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: 0, y: -1 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: 0, y: 1 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: -1, y: 0 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: 1, y: 0 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: -1, y: -1 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: 1, y: -1 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: -1, y: 1 })
            + self.is_beaten_by_neighbor(grid, pos, Vector2DInt { x: 1, y: 1 });

        defeats > 2
    }
}

impl Automata<()> for RockPaperScissors {
    fn update(&mut self) {
        let mut grid_next = self.grid.clone();

        for i in 0..self.grid.len() {
            let pos = grid_index_to_coords(i, GRID_WIDTH, GRID_HEIGHT);

            let next_color = if self.is_beaten(&self.grid, pos) {
                match self.grid[i] {
                    0 => 1,
                    1 => 2,
                    2 => 0,
                    _ => 0,
                }
            } else {
                self.grid[i]
            };

            grid_next[i] = next_color;
        }

        self.grid = grid_next;
    }

    fn before_render(&self) {}

    fn render(&self, _context: &(), i: usize, pixel: &mut [u8]) {
        let viewport_position = viewport_index_to_coords(i, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
        let position = viewport_to_grid(viewport_position, PIXEL_SCALE);
        let index = grid_coords_to_index(position, GRID_WIDTH);

        // Hard-coding to support only 3 colors for now.
        let color = match self.grid[index] {
            0 => [0xff, 0x0, 0x0, 0xff],
            1 => [0x0, 0xff, 0x0, 0xff],
            2 => [0x0, 0x0, 0xff, 0xff],
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
