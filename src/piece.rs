use crate::block::Block;
use crate::rect::Rect;

use macroquad::color::Color;
use macroquad::rand::gen_range;

// 10 x 20
#[derive(Debug)]
pub struct Piece {
    pub shape: Vec<Block>,
    pub center: (i32, i32),
    pub color: Color,
    pub piece_enum: PieceEnum,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceEnum {
    None,
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub trait FindOpen {
    fn find_open(&self) -> Vec<PieceEnum>;
}

// i hate this
impl FindOpen for &[PieceEnum] {
    fn find_open(&self) -> Vec<PieceEnum> {
        let mut all_pieces = vec![
            PieceEnum::I, 
            PieceEnum::O,
            PieceEnum::T,
            PieceEnum::S,
            PieceEnum::Z,
            PieceEnum::J,
            PieceEnum::L,
        ];
        for piece in self.iter() {
            if let PieceEnum::None = piece {
                continue; 
            }
            
            let mut index = 0;
            let mut contains = false;
            for piece_vec in all_pieces.iter() {
                if piece_vec == piece {
                    contains = true;
                    break;
                }
                index += 1;
            }

            if contains {
                all_pieces.remove(index);
            }
        }
        return all_pieces;
    }
}

impl PieceEnum {
    fn random_piece() -> Self {
        let random = gen_range(0, 7);
        //let random = 0;

        let result = match random {
            0 => Self::I,
            1 => Self::O,
            2 => Self::T,
            3 => Self::S,
            4 => Self::Z,
            5 => Self::J,
            6 => Self::L,
            _ => panic!("fish"),
        };

        return result;
    }

    pub fn generate_bag(first_piece: Option<Self>) -> [Self; 8] {
        let mut result = [const{Self::None}; 8];
        let first_piece = match first_piece {
            Some(piece) => piece,
            None => Self::random_piece(),
        };
        result[0] = first_piece;

        for i in 1..=6 {
            let new_piece = 'generate: loop {
                let new_piece = Self::random_piece();
                if !result.contains(&new_piece) {
                    break 'generate new_piece;
                }
            };
            result[i] = new_piece;
        }
        result[7] = PieceEnum::random_piece();

        return result;
    }

    pub fn as_rects(&self) -> (Vec<Rect>, Color) {
        let rects = match self {
            Self::I => {
                (vec![
                    Rect::new(870, 730, 60, 240),
                ], Color::from_rgba(0, 255, 255, 255))  
            },
            Self::O => {
                (vec![
                    Rect::new(840, 730, 120, 120),
                ], Color::from_rgba(255, 255, 0, 255))
            },
            Self::T => {
                (vec![
                    Rect::new(810, 730, 180, 60),
                    Rect::new(870, 790, 60, 60),
                ], Color::from_rgba(255, 0, 255, 255))
            },
            Self::S => {
                (vec![
                    Rect::new(870, 730, 120, 60),
                    Rect::new(810, 790, 120, 60),
                ], Color::from_rgba(0, 255, 0, 255))
            },
            Self::Z => {
                (vec![
                    Rect::new(810, 730, 120, 60),
                    Rect::new(870, 790, 120, 60),
                ], Color::from_rgba(255, 0, 0, 255))
            }, 
            Self::J => {
                (vec![
                    Rect::new(870, 730, 60, 180),
                    Rect::new(810, 850, 60, 60),
                ], Color::from_rgba(0, 0, 255, 255))
            },
            Self::L => {
                (vec![
                    Rect::new(870, 730, 60, 180),
                    Rect::new(930, 850, 60, 60),
                ], Color::from_rgba(255, 128, 0, 255))
            },
            Self::None => {
                (vec![], Color::from_rgba(0, 0, 0, 255))
            },
        };
        rects
    }
}

impl Piece {
    pub fn new(piece_enum: &PieceEnum) -> Self {
        let piece = match piece_enum {
            PieceEnum::I => {
                let color = Color::from_rgba(0, 255, 255, 255);
                let blocks = vec![
                    Block::new((5, -3)),
                    Block::new((5, -4)),
                    Block::new((5, -2)),
                    Block::new((5, -1)),
                ];
                Self { shape: blocks, color: color, center: (5, -3), piece_enum: PieceEnum::I }
            },
            PieceEnum::O => {
                let color = Color::from_rgba(255, 255, 0, 255);
                let blocks = vec![
                    Block::new((4, -2)),
                    Block::new((4, -1)),
                    Block::new((5, -2)),
                    Block::new((5, -1)),
                ];
                Self { shape: blocks, color: color, center: (4, -2), piece_enum: PieceEnum::O}
            },
            PieceEnum::T => {
                let color = Color::from_rgba(255, 0, 255, 255);
                let blocks = vec![
                    Block::new((5, -2)),
                    Block::new((4, -2)),
                    Block::new((6, -2)),
                    Block::new((5, -1)),
                ];
                Self { shape: blocks, color: color, center: (5, -2), piece_enum: PieceEnum::T }
            },
            PieceEnum::S => {
                let color = Color::from_rgba(0, 255, 0, 255);
                let blocks = vec![
                    Block::new((5, -1)),
                    Block::new((4, -1)),
                    Block::new((5, -2)),
                    Block::new((6, -2)),
                ];
                Self { shape: blocks, color: color, center: (5, -1), piece_enum: PieceEnum::S }
            },
            PieceEnum::Z => {
                let color = Color::from_rgba(255, 0, 0, 255);
                let blocks = vec![
                    Block::new((5, -1)),
                    Block::new((4, -2)),
                    Block::new((5, -2)),
                    Block::new((6, -1)),
                ];
                Self { shape: blocks, color: color, center: (5,-1), piece_enum: PieceEnum::Z  }
            },
            PieceEnum::J => {
                let color = Color::from_rgba(0, 0, 255, 255);
                let blocks = vec![
                    Block::new((5, -2)),
                    Block::new((4, -1)),
                    Block::new((5, -1)),
                    Block::new((5, -3)),
                ];
                Self { shape: blocks, color: color, center: (5, -2), piece_enum: PieceEnum::J }
            },
            PieceEnum::L => {
                let color = Color::from_rgba(255, 128, 0, 255);
                let blocks = vec![
                    Block::new((4, -2)),
                    Block::new((4, -3)),
                    Block::new((4, -1)),
                    Block::new((5, -1)),
                ];
                Self { shape: blocks, color: color, center: (4, -2), piece_enum: PieceEnum::L }
            },
            PieceEnum::None => {
                panic!("cannot create a piece from PieceEnum::None");
            }
        };
        return piece;
    }

}
