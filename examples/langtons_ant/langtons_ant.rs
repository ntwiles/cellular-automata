use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    vectors::vector_2d_int::Vector2DInt,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = 200;
const PIXEL_SCALE: u32 = 4;

const VIEWPORT_WIDTH: u32 = GRID_WIDTH * PIXEL_SCALE;
const VIEWPORT_HEIGHT: u32 = GRID_HEIGHT * PIXEL_SCALE;

pub struct LangtonsAnt {
    grid: Vec<bool>,
    ant_pos: Vector2DInt<u32>,
    ant_dir: u8,
}

impl LangtonsAnt {
    pub fn new() -> Self {
        let ant_pos = Vector2DInt {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        };

        Self {
            grid: vec![false; (GRID_WIDTH * GRID_HEIGHT) as usize],
            ant_pos,
            ant_dir: 0,
        }
    }

    fn get_next_position(&self) -> (u32, u32) {
        let (ox, oy) = get_offset_from_direction(self.ant_dir);

        let x = (self.ant_pos.x as i32 + ox) % GRID_WIDTH as i32;
        let y = self.ant_pos.y as i32 + oy % GRID_HEIGHT as i32;

        (x as u32, y as u32)
    }
}

impl Automata<()> for LangtonsAnt {
    fn update(&mut self) {
        let index = grid_coords_to_index(self.ant_pos, GRID_WIDTH);

        if self.grid[index] {
            self.ant_dir = turn_left(self.ant_dir);
        } else {
            self.ant_dir = turn_right(self.ant_dir);
        }

        self.grid[index] = !self.grid[index];

        let (x, y) = self.get_next_position();

        self.ant_pos.x = x;
        self.ant_pos.y = y;
    }

    fn before_render(&self) {}

    fn render(&self, _context: &(), i: usize, pixel: &mut [u8]) {
        let viewport_position = viewport_index_to_coords(i, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
        let position = viewport_to_grid(viewport_position, PIXEL_SCALE);
        let index = grid_coords_to_index(position, GRID_WIDTH);

        let color = if self.grid[index] {
            [0x0, 0x99, 0x11, 0xff]
        } else {
            [0x0, 0x22, 0x11, 0xff]
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

fn turn_right(dir: u8) -> u8 {
    match dir {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 0,
        _ => unreachable!(),
    }
}

fn turn_left(dir: u8) -> u8 {
    match dir {
        0 => 3,
        1 => 0,
        2 => 1,
        3 => 2,
        _ => unreachable!(),
    }
}

fn get_offset_from_direction(dir: u8) -> (i32, i32) {
    match dir {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => unreachable!(),
    }
}
