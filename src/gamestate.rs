extern crate rand;

use std::fmt;
use rand::{Rng, SeedableRng, XorShiftRng};

pub const GRID_HEIGHT: i16 = 900;
pub const GRID_WIDTH: i16 = 1440;

pub const BLOCK_SIZE: i16 = 1;

pub const WINDOW_HEIGHT: i16 = GRID_HEIGHT * BLOCK_SIZE;
pub const WINDOW_WIDTH: i16 = GRID_WIDTH * BLOCK_SIZE;

pub const RAIN_SPARSENESS: i16 = 5;

pub const X: bool = true;
pub const o: bool = false;

#[derive(Eq, PartialEq, Clone)]
pub struct Loc {
    pub x: i16,
    pub y: i16
}

pub struct Map {pub map: Box<[[bool; GRID_HEIGHT as usize]; GRID_WIDTH as usize]>}

pub struct GameState {
    pub particles: Vec<Loc>,
    pub obstacles: Vec<Loc>,
    pub indexes_to_remove: Vec<usize>,
    pub map: Map, //2D array to check occupancy
    pub max_x: i16,
    pub max_y: i16,
    pub rng: XorShiftRng
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "GameState {{").ok();
        writeln!(f, "    num particles = {}", self.particles.len()).ok();
        writeln!(f, "    num obstacles = {}", self.obstacles.len()).ok();
        writeln!(f, "}}")
    }
}

impl Map {
    fn is_occupied(&self, x: i16, y: i16) -> bool {
        // not valid is not occupied to allow particles to leave the grid
        if !GameState::is_valid(x, y) {
            false
        } else {
            unsafe {
                *self.map.get_unchecked(x as usize)
                         .get_unchecked(y as usize)}
            }
    }

    fn get_neighbours(&mut self, x: i16, y: i16) -> [bool; 8] {
        [self.is_occupied(x-1, y-1), self.is_occupied(x, y-1), self.is_occupied(x+1, y-1),
        self.is_occupied(x-1, y), self.is_occupied(x+1, y),
        self.is_occupied(x-1, y+1), self.is_occupied(x, y+1), self.is_occupied(x-1, y+1)]
    }

    fn add_coord_map(&mut self, x: i16, y: i16) {
        self.map[x as usize][y as usize] = true;
    }

    fn remove_coord_map(&mut self, x: i16, y: i16) {
        self.map[x as usize][y as usize] = false;
    }
}

impl GameState {
    //check if point in inside the grid
    pub fn is_valid(x: i16, y: i16) -> bool {
        if x<0 || x>=GRID_WIDTH || y<0 || y>=GRID_HEIGHT {false} else {true}
    }

    pub fn update(&mut self) {
        //MAIN LOGIC
        for (index, particle) in self.particles.iter_mut().enumerate() {
            let (x, y) = (particle.x, particle.y);
            let neighbours = self.map.get_neighbours(x, y);
            let (x_new, y_new) = match neighbours {
               /*above  side below*/
                [_,_,_, _,_, _,o,_] => ( x, y+1 ),
                [_,X,_, o,X, _,X,_] => ( x-1, y ),
                [_,X,_, X,o, _,X,_] => ( x+1, y ),
                [_,X,_, o,o, _,X,_] => ( if y&2 == 0 {x-1} else {x+1}, y ), //deterministic choice
                //[_,o,_, X,X, X,X,X] => ( x, y-1 ), //boiling sand
                //[_,X,_, o,o, _,X,_] => ( if self.rng.gen::<i16>()&2 == 0 {x-1} else {x+1}, y ), //random choice
                                        _ => continue,
            };
            if Self::is_valid(x_new, y_new) {
                self.map.remove_coord_map(x, y);
                self.map.add_coord_map(x_new, y_new);
                *particle = Loc {x: x_new, y: y_new};
            } else {
                self.map.map[x as usize][y as usize] = false;
                self.indexes_to_remove.push(index); //prepare list of particles to remove
            }
        }
        //must use .rev(), otherwise you can't remove the last element
        for index in self.indexes_to_remove.iter().rev() {
            self.particles.swap_remove(*index);
        }
        self.indexes_to_remove.clear();
    }//end update

    pub fn rain(&mut self) {
        for x in 1*(GRID_WIDTH/20)..19*(GRID_WIDTH/20) {
            if !self.map.is_occupied(x, 0) && self.rng.gen::<i16>()&RAIN_SPARSENESS == 0 {
                self.particles.push(Loc {x: x, y: 0});
                self.map.add_coord_map(x, 0);
            }
        }
    }

    fn remove_indices_in_rect(items: &mut Vec<Loc>,
                              indexes_to_remove: &mut Vec<usize>,
                              ul: Loc, lr: Loc) {
        for (index, p) in items.iter().enumerate() {
            if p.x >= ul.x && p.y >= ul.y && p.x < lr.x && p.y < lr.y {
                indexes_to_remove.push(index);
            }
        }

        for index in indexes_to_remove.iter().rev() {
            items.swap_remove(*index);
        }

        indexes_to_remove.clear();
    }

    fn add_particle (&mut self, x: i16, y: i16) {
        self.map.add_coord_map(x, y);
        let loc = Loc {x: x, y: y};
        self.particles.push(loc);
    }

    fn add_obstacle (&mut self, x: i16, y: i16) {
        self.map.add_coord_map(x, y);
        let loc = Loc {x: x, y: y};
        self.obstacles.push(loc);
    }

    pub fn remove_square (&mut self, ux: i16, uy: i16, dx: i16, dy: i16) {
        if Self::is_valid(ux + dx, uy + dy) {
            for x in ux..ux+dx {
                for y in uy..uy+dy {
                    self.map.remove_coord_map(x, y);
                }
            }

            let ul = Loc {x: ux, y: uy};
            let lr = Loc {x: ux + dx, y: uy + dy};
            Self::remove_indices_in_rect(&mut self.obstacles,
                                              &mut self.indexes_to_remove,
                                              ul.clone(), lr.clone());
            Self::remove_indices_in_rect(&mut self.particles,
                                              &mut self.indexes_to_remove,
                                              ul, lr);
        }
    }

    pub fn paint_square_obstacles (&mut self, ux: i16, uy: i16, dx: i16, dy: i16) {
        if Self::is_valid(ux + dx, uy + dy) {
            for x in ux..ux+dx {
                for y in uy..uy+dy {
                    if !self.map.is_occupied(x, y) {
                        self.map.add_coord_map(x, y);
                        self.add_obstacle(x, y);
                    }
                }
            }
        }
    }

    pub fn new() -> GameState {
        GameState {
                particles: Vec::with_capacity(10_000),
                obstacles: Vec::with_capacity(10_000),
                indexes_to_remove: Vec::with_capacity(1000),
                map: Map { map: box [[false; GRID_HEIGHT as usize]; GRID_WIDTH as usize] },
                max_x: GRID_WIDTH,
                max_y: GRID_HEIGHT,
                rng: SeedableRng::from_seed([1, 2, 3, 4])
            }
    }

}//end impl GameState
