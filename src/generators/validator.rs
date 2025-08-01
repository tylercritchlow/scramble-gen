use crate::moves::{Move, MoveFace, MoveType, MoveWidth};
use rand::Rng;

pub fn can_combine(first: &Move, second: &Move) -> bool {
    first.move_face == second.move_face && first.move_width == second.move_width
}

pub fn combine_moves(first: &Move, second: &Move) -> Option<Move> {
    if !can_combine(first, second) {
        panic!("Cannot combine moves on different faces or widths");
    }

    let combined_type = match (first.move_type, second.move_type) {
        (MoveType::Normal, MoveType::Normal) => Some(MoveType::Double),
        (MoveType::Normal, MoveType::Double) => Some(MoveType::Prime),
        (MoveType::Normal, MoveType::Prime) => None,
        (MoveType::Double, MoveType::Normal) => Some(MoveType::Prime),
        (MoveType::Double, MoveType::Double) => None,
        (MoveType::Double, MoveType::Prime) => Some(MoveType::Normal),
        (MoveType::Prime, MoveType::Normal) => None,
        (MoveType::Prime, MoveType::Double) => Some(MoveType::Normal),
        (MoveType::Prime, MoveType::Prime) => Some(MoveType::Double),
    };

    combined_type.map(|move_type| Move {
        move_face: first.move_face,
        move_type,
        move_width: first.move_width,
    })
}

fn would_create_parallel_conflict(moves: &[Move], new_move: &Move) -> bool {
    if moves.len() < 2 {
        return false;
    }

    let last_move = &moves[moves.len() - 1];
    let second_last = &moves[moves.len() - 2];

    if second_last.move_face == new_move.move_face
        && second_last.move_face.same_axis(&last_move.move_face)
    {
        return true;
    }

    false
}

pub fn generate_validated_scramble<R: Rng>(
    rng: &mut R,
    length: usize,
    width_selector: impl Fn(&mut R) -> MoveWidth,
) -> Vec<Move> {
    let mut scramble_moves = Vec::new();

    while scramble_moves.len() < length {
        let move_face: MoveFace = rng.random();
        let move_type: MoveType = rng.random();
        let move_width = width_selector(rng);

        let new_move = Move {
            move_face,
            move_type,
            move_width,
        };

        if let Some(last_move) = scramble_moves.last()
            && can_combine(last_move, &new_move)
        {
            if let Some(combined) = combine_moves(last_move, &new_move) {
                scramble_moves.pop();
                scramble_moves.push(combined);
            } else {
                scramble_moves.pop();
            }
            continue;
        }

        if would_create_parallel_conflict(&scramble_moves, &new_move) {
            continue;
        }

        scramble_moves.push(new_move);
    }

    scramble_moves
}
