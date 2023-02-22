use std::mem;

use crate::{
    board::{Board, CheckerError, PieceColor, PieceType, SlotContent},
    move_info::{Coordinate, MoveInfo},
};

pub struct LegalMovesComputer<'a> {
    moves: Vec<MoveInfo>,
    board: &'a Board,
}

impl<'a> LegalMovesComputer<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            moves: vec![],
            board,
        }
    }

    fn add_if_empty() {}

    pub fn compute(&mut self, coordinate: Coordinate) -> Result<Vec<MoveInfo>, CheckerError> {
        coordinate.check()?;

        let cell = self.board.at(&coordinate);
        if let SlotContent::Piece(piece) = cell {
            let y_increments: Vec<isize> = match piece.piece_type {
                PieceType::King => vec![-1, 1],
                PieceType::Man => match piece.color {
                    PieceColor::White => vec![1],
                    PieceColor::Black => vec![-1],
                },
            };

            for y_increment in y_increments {
                for i in 0..=1 {
                    let mut to = coordinate;
                    to.y = (to.y as isize + y_increment) as usize;

                    if i == 0 {
                        to.x += 1;
                    } else {
                        to.x -= 1;
                    }

                    if to.check().is_err() {
                        continue;
                    }

                    let cell = self.board.at(&to);
                    if let SlotContent::Piece(other_piece) = cell {
                        if piece.color == other_piece.color {
                            continue;
                        }

                        for i in 0..=1 {
                            let mut behind = to;
                            behind.y = (to.y as isize + y_increment) as usize;

                            if i == 0 {
                                behind.x += 1;
                            } else {
                                behind.x -= 1;
                            }

                            if behind.check().is_err() {
                                continue;
                            }

                            let cell = self.board.at(&behind);
                            if matches!(cell, SlotContent::Empty) {
                                // TODO: add multiple captures in a row
                                self.moves.push(MoveInfo {
                                    from: coordinate,
                                    to,
                                    capturing: vec![to],
                                });
                            }
                        }
                    } else {
                        self.moves.push(MoveInfo {
                            from: coordinate,
                            to,
                            capturing: vec![],
                        });
                    }
                }
            }
        } else {
            return Err(CheckerError::EmptyCell(coordinate));
        }

        Ok(mem::take(&mut self.moves))
    }
}
