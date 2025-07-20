use scramble_gen::cube::Cube;
use scramble_gen::generators::validator::{can_combine, combine_moves};
use scramble_gen::moves::{Move, MoveFace, MoveType, MoveWidth};
use scramble_gen::scramble::Scramble;

fn check_redundant_moves(first: &Move, second: &Move) -> bool {
    // Check if moves are on the same face and width
    if first.move_face == second.move_face && first.move_width == second.move_width {
        // These sequences are redundant and should have been combined:
        // R R -> R2
        // R R2 -> R'
        // R' R -> should cancel
        // etc.
        return true;
    }
    false
}

fn verify_no_redundant_moves(scramble: &Scramble) -> Result<(), String> {
    let moves = &scramble.moves;

    for i in 0..moves.len().saturating_sub(1) {
        if check_redundant_moves(&moves[i], &moves[i + 1]) {
            return Err(format!(
                "Found redundant moves at position {}: {} followed by {}",
                i,
                moves[i],
                moves[i + 1]
            ));
        }
    }

    Ok(())
}

#[test]
fn test_no_redundant_moves_3x3() {
    for i in 0..1000 {
        let scramble = Scramble::generate(Cube::ThreeByThree, Some(25));
        if let Err(e) = verify_no_redundant_moves(&scramble) {
            panic!("3x3 scramble {i} failed: {e}");
        }
    }
}

#[test]
fn test_no_redundant_moves_4x4() {
    for i in 0..1000 {
        let scramble = Scramble::generate(Cube::FourByFour, Some(40));
        if let Err(e) = verify_no_redundant_moves(&scramble) {
            panic!("4x4 scramble {i} failed: {e}");
        }
    }
}

#[test]
fn test_no_redundant_moves_5x5() {
    for i in 0..1000 {
        let scramble = Scramble::generate(Cube::FiveByFive, Some(60));
        if let Err(e) = verify_no_redundant_moves(&scramble) {
            panic!("5x5 scramble {i} failed: {e}");
        }
    }
}

#[test]
fn test_no_redundant_moves_6x6() {
    for i in 0..1000 {
        let scramble = Scramble::generate(Cube::SixBySix, Some(80));
        if let Err(e) = verify_no_redundant_moves(&scramble) {
            panic!("6x6 scramble {i} failed: {e}");
        }
    }
}

#[test]
fn test_no_redundant_moves_7x7() {
    for i in 0..1000 {
        let scramble = Scramble::generate(Cube::SevenBySeven, Some(100));
        if let Err(e) = verify_no_redundant_moves(&scramble) {
            panic!("7x7 scramble {i} failed: {e}");
        }
    }
}

#[test]
fn test_combine_moves_normal_normal() {
    let move1 = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Single,
    };
    let move2 = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Single,
    };

    let combined = combine_moves(&move1, &move2);
    assert!(combined.is_some());
    assert_eq!(combined.unwrap().move_type, MoveType::Double);
}

#[test]
fn test_combine_moves_normal_prime_cancels() {
    let move1 = Move {
        move_face: MoveFace::Up,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Single,
    };
    let move2 = Move {
        move_face: MoveFace::Up,
        move_type: MoveType::Prime,
        move_width: MoveWidth::Single,
    };

    let combined = combine_moves(&move1, &move2);
    assert!(combined.is_none());
}

#[test]
fn test_combine_moves_double_double_cancels() {
    let move1 = Move {
        move_face: MoveFace::Front,
        move_type: MoveType::Double,
        move_width: MoveWidth::Single,
    };
    let move2 = Move {
        move_face: MoveFace::Front,
        move_type: MoveType::Double,
        move_width: MoveWidth::Single,
    };

    let combined = combine_moves(&move1, &move2);
    assert!(combined.is_none());
}

#[test]
fn test_combine_moves_prime_prime() {
    let move1 = Move {
        move_face: MoveFace::Left,
        move_type: MoveType::Prime,
        move_width: MoveWidth::Single,
    };
    let move2 = Move {
        move_face: MoveFace::Left,
        move_type: MoveType::Prime,
        move_width: MoveWidth::Single,
    };

    let combined = combine_moves(&move1, &move2);
    assert!(combined.is_some());
    assert_eq!(combined.unwrap().move_type, MoveType::Double);
}

#[test]
fn test_can_combine_different_faces() {
    let move1 = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Single,
    };
    let move2 = Move {
        move_face: MoveFace::Up,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Single,
    };

    assert!(!can_combine(&move1, &move2));
}

#[test]
fn test_can_combine_different_widths() {
    let move1 = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Single,
    };
    let move2 = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Wide,
    };

    assert!(!can_combine(&move1, &move2));
}
