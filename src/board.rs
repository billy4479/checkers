use crate::{
    legal_moves::LegalMovesComputer,
    move_info::{Coordinate, MoveInfo},
};
use colored::*;
use std::fmt::Display;
use thiserror::Error;

#[derive(Copy, Clone)]
pub enum SlotContent {
    Empty,
    Piece(Piece),
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

#[derive(Copy, Clone)]
pub enum PieceType {
    Man,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Error, Debug)]
pub enum CheckerError {
    #[error("Moving {0} is illegal: {1}")]
    IllegalMove(MoveInfo, String),

    #[error("Coordinate {0} is invalid")]
    InvalidCoordinate(Coordinate),

    #[error("Cell {0} is empty")]
    EmptyCell(Coordinate),
}

pub struct Board {
    cells: [[SlotContent; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            cells: [[SlotContent::Empty; 8]; 8],
        };

        let mut fill_row = |row_index: usize, color: PieceColor| {
            board.cells[row_index]
                .iter_mut()
                .enumerate()
                .for_each(|(i, x)| {
                    if (i + row_index) % 2 == 0 {
                        *x = SlotContent::Piece(Piece {
                            piece_type: PieceType::Man,
                            color,
                        })
                    }
                });
        };

        for i in 0..=2 {
            fill_row(i, PieceColor::White);
        }

        for i in 5..=7 {
            fill_row(i, PieceColor::Black);
        }

        board
    }

    fn at_mut(&mut self, coordinate: &Coordinate) -> &mut SlotContent {
        &mut self.cells[coordinate.y][coordinate.x]
    }

    pub fn at(&self, coordinate: &Coordinate) -> &SlotContent {
        &self.cells[coordinate.y][coordinate.x]
    }

    pub fn move_piece(&mut self, move_info: MoveInfo) -> Result<(), CheckerError> {
        move_info.from.check()?;
        move_info.to.check()?;

        let legal_moves = LegalMovesComputer::new(self).compute(move_info.from)?;

        println!("{legal_moves:?}");

        match legal_moves.iter().find(|x| x.to == move_info.to) {
            None => Err(CheckerError::IllegalMove(move_info, "?".into())),
            Some(x) => {
                let old = *self.at_mut(&move_info.from);
                *self.at_mut(&x.from) = SlotContent::Empty;
                *self.at_mut(&x.to) = old;

                x.capturing.iter().for_each(|captured| {
                    *self.at_mut(captured) = SlotContent::Empty;
                });

                Ok(())
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate().rev() {
            write!(f, "\t{y}")?;
            for (x, cell) in row.iter().enumerate() {
                let aaa = match cell {
                    SlotContent::Piece(piece) => {
                        let char = match piece.piece_type {
                            PieceType::Man => "o",
                            PieceType::King => "O",
                        };
                        match piece.color {
                            PieceColor::Black => char.black(),
                            PieceColor::White => char.white(),
                        }
                    }
                    SlotContent::Empty => " ".white(),
                };

                if (x + y) % 2 == 0 {
                    write!(f, "{}", aaa.on_bright_black())?;
                } else {
                    write!(f, "{}", aaa.on_bright_white())?;
                }
            }
            writeln!(f)?;
        }

        write!(f, "\t ")?;
        for i in 0..8 {
            write!(f, "{i}")?;
        }
        writeln!(f)?;

        Ok(())
    }
}
