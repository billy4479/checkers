use checkers::{
    board::Board,
    move_info::{Coordinate, MoveInfo},
};

fn main() {
    let mut board = Board::new();
    println!("{board}");

    let result = board.move_piece(MoveInfo {
        from: Coordinate { x: 2, y: 2 },
        to: Coordinate { x: 3, y: 3 },
        capturing: vec![],
    });

    match result {
        Ok(()) => println!("{board}"),
        Err(e) => println!("{e}"),
    };
}
