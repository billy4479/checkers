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

    fn add_if_empty(&mut self, from: &Coordinate, to: &Coordinate) {
        let cell = self.board.at(&to);
        match cell {
            SlotContent::Empty => {
                self.moves.push(MoveInfo {
                    from: from.clone(),
                    to: from.clone(),
                    capturing: vec![],
                });
            }
            SlotContent::Piece(_) => {}
        }
    }

    fn get_move_locations(
        &self,
        coordinate: &Coordinate,
        piece_type: PieceType,
        color: PieceColor,
    ) -> Vec<Coordinate> {
        let mut moves = vec![];

        let y_increments: Vec<isize> = match piece_type {
            PieceType::King => vec![-1, 1],
            PieceType::Man => match color {
                PieceColor::White => vec![1],
                PieceColor::Black => vec![-1],
            },
        };

        for i in 0..=1 {
            let mut coord = coordinate.clone();
            if i == 0 {
                coord.x += 1;
            } else {
                coord.x -= 1;
            }

            for y_increment in &y_increments {
                let mut coord = coord.clone();
                coord.y = (coord.y as isize + y_increment) as usize;
                if coord.check().is_err() {
                    continue;
                }

                moves.push(coord);
            }
        }

        moves
    }

    fn add_capture(&mut self, from: Coordinate, to: Coordinate, color: PieceColor) {
        let cell = self.board.at(&to);
        if let SlotContent::Piece(other_piece) = cell {
            if color == other_piece.color {
                return;
            }
        }
    }

    pub fn compute(&mut self, from: &Coordinate) -> Result<Vec<MoveInfo>, CheckerError> {
        from.check()?;

        let cell = self.board.at(from);
        if let SlotContent::Piece(piece) = cell {
            let moves = self.get_move_locations(from, piece.piece_type, piece.color);

            for to in &moves {
                self.add_if_empty(from, &to);

                // let cell = self.board.at(&to);
                // if let SlotContent::Piece(other_piece) = cell {
                //     if piece.color == other_piece.color {
                //         continue;
                //     }

                //     let cell = self.board.at(&to);
                //     if matches!(cell, SlotContent::Empty) {
                //         // TODO: add multiple captures in a row
                //         self.moves.push(MoveInfo {
                //             from: from.clone(),
                //             to: ,
                //             capturing: vec![to],
                //         });
                //     }
                // }
            }
        } else {
            return Err(CheckerError::EmptyCell(from.clone()));
        }

        Ok(mem::take(&mut self.moves))
    }
}
