use cellular_automata::{
    automata::Automata,
    grid::grid_coords_to_index,
    viewport::{viewport_index_to_coords, viewport_to_grid},
};

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = 200;
const PIXEL_SCALE: u32 = 4;

const VIEWPORT_WIDTH: u32 = GRID_WIDTH * PIXEL_SCALE;
const VIEWPORT_HEIGHT: u32 = GRID_HEIGHT * PIXEL_SCALE;

pub struct LangtonsAnt {
    grid: Vec<bool>,
    ant_x: u32,
    ant_y: u32,
    ant_dir: u8,
}

impl LangtonsAnt {
    pub fn new() -> Self {
        Self {
            grid: vec![false; (GRID_WIDTH * GRID_HEIGHT) as usize],
            ant_x: GRID_WIDTH / 2,
            ant_y: GRID_HEIGHT / 2,
            ant_dir: 0,
        }
    }

    fn get_next_position(&self) -> (u32, u32) {
        let (ox, oy) = get_offset_from_direction(self.ant_dir);

        let x = (self.ant_x as i32 + ox) % GRID_WIDTH as i32;
        let y = self.ant_y as i32 + oy % GRID_HEIGHT as i32;

        (x as u32, y as u32)
    }
}

impl Automata for LangtonsAnt {
    fn update(&mut self) {
        let index = grid_coords_to_index(self.ant_x, self.ant_y, GRID_WIDTH);

        if self.grid[index] {
            self.ant_dir = turn_left(self.ant_dir);
        } else {
            self.ant_dir = turn_right(self.ant_dir);
        }

        self.grid[index] = !self.grid[index];

        let (x, y) = self.get_next_position();

        self.ant_x = x;
        self.ant_y = y;
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
