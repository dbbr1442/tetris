use macroquad::time::get_time;

use crate::piece::{Piece, PieceEnum};
use crate::piece::FindOpen;

use crate::block::Block;

const QUARTER: f64 = std::f64::consts::TAU/4.0; // 90 degrees as radians

pub enum MoveDirection {
    Left, 
    Right,
    Down,
}

#[derive(Debug)]
pub struct Game {
    pub inplay: Piece,
    pub playfield: Vec<Piece>,
    border: Vec<Block>,
    pub score: u32,
    bag: [PieceEnum; 8],
    bag_i: usize,
    pub lost: bool,
    //pub timer: f64,
    pub timer: f64,
}

impl Game {
    pub fn rotate_piece(&mut self) {
        let center = self.inplay.center; 
        let mut new_shape: Vec<Block> = Vec::new(); 

        let mut collision = false;
        for block in self.inplay.shape.iter_mut() {
            let (old_x, old_y) = block.location;

            if (old_x == center.0) && (old_y == center.1) {
                new_shape.push(Block::new(center));
                continue; // the center doesnt have to be rotated
            }

            let new_x = ((old_x-center.0) as f64*QUARTER.cos())-((old_y-center.1) as f64*QUARTER.sin());
            let new_y = ((old_x-center.0) as f64*QUARTER.sin())-((old_y-center.1) as f64*QUARTER.cos());
            let result = (new_x.round() as i32+center.0, new_y.round() as i32+center.1);

            for piece in self.playfield.iter() {
                for block in piece.shape.iter() {
                    if block.location == result {
                        collision = true;
                    }
                }
            }

            for block in self.border.iter() {
                if block.location == result {
                    collision = true;
                }
            }
            new_shape.push(Block::new(result));
        } 

        if collision == false {
            self.inplay.shape = new_shape;
        }
    }

    /// false = no collision, true = collision
    pub fn move_piece(&mut self, direction: MoveDirection) -> bool {
        let move_vector = match direction {
            MoveDirection::Left => (-1, 0),
            MoveDirection::Right => (1, 0),
            MoveDirection::Down => (0, 1),
        };
        let mut new_shape: Vec<Block> = Vec::new();
        let new_center = (self.inplay.center.0+move_vector.0, self.inplay.center.1+move_vector.1);
        
        let mut collision = false;
        for block in self.inplay.shape.iter_mut() {
            let new_location = (block.location.0+move_vector.0, block.location.1+move_vector.1);

            for piece in self.playfield.iter() {
                for block in piece.shape.iter() {
                    if block.location == new_location {
                        collision = true;
                    }
                }
            }

            for block in self.border.iter() {
                if block.location == new_location {
                    collision = true;
                }
            }
            new_shape.push(Block::new(new_location));

        }
        if collision == false {
            self.inplay.shape = new_shape;
            self.inplay.center = new_center;
        }

        return collision;

    }

    pub fn check_lose(&mut self) {
        for piece in self.playfield.iter() {
            for block in piece.shape.iter() {
                if block.location.1 < 0 {
                    //return true;
                    self.lost = true;
                    return;
                }
            }
            
        }
        //return false;
    }

    pub fn move_board_down(&mut self, if_above: i32) {
        for piece in self.playfield.iter_mut() {
            for block in piece.shape.iter_mut() {
                if block.location.1 <= if_above {
                    block.location = (block.location.0, block.location.1+1);
                }
            }
        }
    }

    pub fn remove_line(&mut self, line: i32) {
        for piece in self.playfield.iter_mut() {
            piece.shape.retain(|element| {element.location.1 != line});
        }
    }

    pub fn check_line(&mut self) {
        let mut piece_map: Vec<Vec<bool>> = vec![vec![false; 10]; 24];


        for piece in self.playfield.iter() {
            for block in piece.shape.iter() {
                piece_map[(block.location.1+4) as usize][(block.location.0) as usize] = true;
            }
        } 

        let mut cline = -4;
        for line in piece_map.iter() {
            let mut line_clear = true;
            for element in line.iter() {
                if *element == false {
                    line_clear = false;
                }
            }

            if line_clear == true {
                self.remove_line(cline);
                self.move_board_down(cline);
                self.score += 100;
                //return true;
            }
            cline += 1;
        }
        //return false;
    }

    pub fn new_game() -> Self {
        let mut border: Vec<Block> = Vec::new();
        for y in -4..=19 {
            let block_left = Block::new((-1, y));
            let block_right = Block::new((10, y));
            border.push(block_left);
            border.push(block_right);
        }

        for x in 0..=9 {
            let block = Block::new((x, 20));
            border.push(block);
        }

        let bag = PieceEnum::generate_bag(None);

        Self { 
            inplay: Piece::new(&bag[0]), 
            playfield: Vec::new(), 
            border: border, 
            score: 0, 
            bag: bag,
            bag_i: 1,
            lost: false,
            //timer: 0.0,
            timer: get_time(),
        }
    }

    pub fn next_piece(&mut self) -> PieceEnum {
        let next_piece = if self.bag_i >= 7 {
            let next_piece = self.bag[self.bag_i];
            self.bag = PieceEnum::generate_bag(Some(self.bag[7]));
            self.bag_i = 1;
            next_piece
        } else {
            self.bag[self.bag_i]
        };
        next_piece
    }

    pub fn place_piece(mut self) -> Self {
        let inplay = self.inplay;
        self.playfield.push(inplay);

        let next_piece = if self.bag_i >= 7 {
            let next_piece = self.bag[self.bag_i];
            self.bag = PieceEnum::generate_bag(Some(self.bag[7]));
            self.bag_i = 1;
            next_piece
        } else {
            self.bag[self.bag_i]
        };
        
        self.inplay = Piece::new(&next_piece);
        self.bag_i += 1;

        return self;
    }

    //[J, O, S, I, T, L, Z, I], 6 //place
    //[J, O, S, I, T, L, Z, I], 7
    //[I, L, Z, S, J, O, T, S], 1 //borrow next one turns into what we currently have therefore we
    //must check to randomize the currently existing one we already have
    //[L, Z, Z, S, J, O, T, S], 1

    // there's gotta be a better way to do this
    pub fn hold_piece(mut self) -> Self {
        let current_piece = self.inplay.piece_enum;

        let next_piece = self.next_piece();

        self.bag[self.bag_i-1] = next_piece;
        self.bag[self.bag_i] = current_piece;

        if self.bag_i == 1 {
            let mut none = 0;
            for i in 2..=6 {
                if self.bag[i] == self.bag[1] {
                    self.bag[i] = PieceEnum::None;
                    none = i;
                    break;
                }
            }

            let slice = &self.bag[1..=6];
            let open = slice.find_open();
            let set_piece = match open.first() {
                None => panic!(),
                Some(piece) => piece,
            };

            self.bag[none] = *set_piece;
        }

        self.inplay = Piece::new(&self.bag[self.bag_i-1]);

        return self;
    }

}
