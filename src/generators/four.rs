use crate::moves::{Move, MoveFace, MoveType, MoveWidth};
use crate::scramble::Scramble;
use rand::{Rng, rng};

pub fn generate(length: Option<usize>) -> Scramble {
    let mut scramble_moves = Vec::new();
    let mut rng = rng();

    for _ in 0..length.unwrap_or(40) {
        let move_face: MoveFace = rng.random();
        let move_type: MoveType = rng.random();

        let move_width = match rng.random_range(0..10) {
            0..=7 => MoveWidth::Single, // 80% single moves
            _ => MoveWidth::Wide,       // 20% wide moves
        };

        let move_ = Move {
            move_face,
            move_type,
            move_width,
        };

        scramble_moves.push(move_);
    }
    Scramble {
        moves: scramble_moves,
    }
}
