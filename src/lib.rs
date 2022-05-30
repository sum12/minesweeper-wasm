use std::fmt::Display;

pub type Point = (usize, usize);

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum CellContent {
    Mine,
    NoMine(u8),
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Close = 0,
    Open = 1,
    Flag = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Cell {
    state: CellState,
    has_mine: bool,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    state: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub finished: bool,
}

pub fn neighbour_points(point: Point, bounds: Point) -> impl Iterator<Item = Point> {
    (point.0.checked_sub(1).unwrap_or(0)..=std::cmp::min(point.0 + 1, bounds.0 - 1))
        .map(move |x| {
            (point.1.checked_sub(1).unwrap_or(0)..=std::cmp::min(point.1 + 1, bounds.1 - 1))
                .map(move |y| ((x, y)))
        })
        .flatten()
}

impl Game {
    pub fn to_array(&self) -> Vec<u8> {
        self.state.iter().map(|cell| 0).collect()
    }

    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = &Cell> {
        neighbour_points(point, (self.width, self.height))
            .map(|(x, y)| &self.state[y * self.height + x])
    }

    pub fn neighbours_count(&self, p: Point, state: CellState) -> usize {
        self.neighbours(p).filter(|c| c.state == state).count()
    }

    pub fn add_mine(&mut self, p: Point) {
        self.state[p.1 * self.height + p.0].has_mine = true;
    }

    pub fn has_mine(&mut self, p: Point) -> bool {
        self.state[p.1 * self.height + p.0].has_mine
    }

    pub fn is_closed(&self, p: Point) -> bool {
        matches!(self.state[p.1 * self.height + p.0].state, CellState::Close)
    }

    pub fn open_neighbours(&mut self, p: Point) {
        for nei in neighbour_points(p, (self.width, self.height)) {
            if self.is_closed(nei) {
                self.open(nei);
            }
        }
    }

    pub fn mine_count(&self, p: Point) -> usize {
        self.neighbours(p).filter(|c| c.has_mine).count()
    }

    pub fn toggle_flag(&mut self, p: Point) {
        if self.finished {
            return;
        }
        let cstate = &mut self.state[p.1 * self.height + p.0].state;

        *cstate = match cstate {
            CellState::Flag => CellState::Close,
            CellState::Close => CellState::Flag,
            CellState::Open => panic!("cannot flag open cell"),
        };
    }
    pub fn open(&mut self, p: Point) {
        if self.finished {
            return;
        }
        let cell = &mut self.state[p.1 * self.height + p.0];
        match cell.state {
            CellState::Flag => {
                cell.state = CellState::Flag;
            }
            CellState::Open => {
                let mc = self.mine_count(p);
                let fc = self.neighbours_count(p, CellState::Flag);
                if mc == fc {
                    self.open_neighbours(p)
                }
            }
            CellState::Close => {
                cell.state = CellState::Open;
                self.finished = cell.has_mine;
                let mc = self.mine_count(p);
                if mc == 0 {
                    self.open_neighbours(p)
                }
            }
        }
        if self
            .state
            .iter()
            .all(|cell| matches!(cell.state, CellState::Open | CellState::Flag))
        {
            self.finished = true;
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

fn random_mine_position(max: usize) -> usize {
    (random() * max as f32).floor() as usize
}

#[wasm_bindgen]
impl Game {
    pub fn cell_open(&mut self, x: usize, y: usize) {
        self.open((x, y))
    }

    pub fn cell_toggle_flag(&mut self, x: usize, y: usize) {
        self.toggle_flag((x, y))
    }

    pub fn cells(&self) -> *const Cell {
        self.state.as_ptr()
    }
    pub fn new(width: usize, height: usize, mines: u32) -> Self {
        let mut g = Game {
            state: vec![
                Cell {
                    state: CellState::Close,
                    has_mine: false,
                };
                width * height
            ],
            width,
            height,
            finished: false,
        };
        let mut mc = 0;
        while mc < mines {
            let (x, y) = (random_mine_position(width), random_mine_position(height));
            if g.has_mine((x, y)) {
                continue;
            }
            g.add_mine((x, y));
            mc += 1
        }
        g
    }

    pub fn add_mines(&mut self, x: u32, y: u32) {
        self.state[y as usize * self.height + x as usize].has_mine = true;
    }

    pub fn cell_minecount(&self, x: u32, y: u32) -> usize {
        self.mine_count((x as usize, y as usize))
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, cells) in self.state.chunks(self.width).enumerate() {
            if y == 0 {
                f.write_str(&format!("{:_>1$}\n", "", 2 + self.width * 2))?;
            }
            f.write_str("|")?;
            for (x, cell) in cells.iter().enumerate() {
                let c = if cell.has_mine {
                    '■'
                } else if cell.state == CellState::Open {
                    char::from_u32(self.mine_count((x, y)) as u32 + '0' as u32).unwrap()
                } else if cell.state == CellState::Flag {
                    'F'
                } else {
                    ' '
                };
                f.write_fmt(format_args!("{:2}", c))?;
            }
            f.write_str("|\n")?;
        }
        f.write_str(&format!("{:‾>1$}\n", "", 2 + self.width * 2))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_test() {
        let mut g = Game::new(10, 10, 5);
        g.add_mine((1, 1));
        g.add_mine((5, 5));
        println!("{}", g);
        g.toggle_flag((8, 8));
        println!("{}", g);
        g.open((1, 5));
        println!("{}", g);
        assert_eq!(1, 0)
    }
}
